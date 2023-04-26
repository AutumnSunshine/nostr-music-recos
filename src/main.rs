use std::time::Duration;
use nostr_sdk::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use std::str::FromStr;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {
    
    //Enter your secret key here that looks like nsec1...
    let my_keys = Keys::from_sk_str("Your secret key")?;

    // Show bech32 public key
    let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);
    
    // Create new client with your secret key
    let client = Client::new(&my_keys);

    //Set your start date & time and the end date & time
    let since_time: NaiveDateTime = NaiveDate::from_ymd_opt(2023, 04, 25).unwrap().and_hms_opt(00, 00, 01).unwrap();
    let until_time: NaiveDateTime = NaiveDate::from_ymd_opt(2023, 04, 25).unwrap().and_hms_opt(23, 59, 59).unwrap();
    let sc_time = Timestamp::from_str(&since_time.timestamp().to_string());
    let ut_time = Timestamp::from_str(&until_time.timestamp().to_string());    

    
    // Add relays -- only nostr.band as it supports filters
    client.add_relay("wss://relay.nostr.band",None).await?;    

    // Connect to relays
    client.connect().await;
    
    let filter1 = Filter::new()
        .search("music.apple.com")   // search Apple music tracks
        .kind(Kind::TextNote)
        .since(sc_time?)
        .until(ut_time?);
    
    let filter2 = Filter::new()
        .search("open.spotify.com/track")
        .kind(Kind::TextNote)   // search spotify tracks
        .since(Timestamp::from_str(&since_time.timestamp().to_string())?)
        .until(Timestamp::from_str(&until_time.timestamp().to_string())?);

    let filter3 = Filter::new()
        .search("tidal.com/track")   // Search Tidal tracks
        .kind(Kind::TextNote)
        .since(Timestamp::from_str(&since_time.timestamp().to_string())?)
        .until(Timestamp::from_str(&until_time.timestamp().to_string())?);
    

    let timeout = Duration::from_secs(30);

    //Creating a vector with the 3 filters and sending the filter query to the relay, with a timeout in case nothing is returned
    let events = client.get_events_of(vec![filter1, filter2, filter3], Some(timeout)).await.unwrap();
    println!("Number of events: {}", events.len());
    
    /*Declaring variables for extracting music tracks*/
    
    //Patterns
    let pubkey_re = Regex::new(r#""pubkey":"([0-9a-f]+)"#).unwrap();  //for extracting pubkey from JSON
    let content_re = Regex::new(r#""content":"(.+?)","#).unwrap(); //for extracting content from JSON
    let spotify_re = Regex::new(r#"\w*https://open.spotify.com/track/(?P<trak>[0-9A-Za-z]+)[\s|\?]*[.]*"#).unwrap(); //for matching spotify tracks from content
    let apple_re = Regex::new(r#"\w*https://music.apple.com/(?P<trak>[0-9A-Za-z/-]+)[\s|\?]*[.]*"#).unwrap(); //for matching apple music tracks from content
    let tidal_re = Regex::new(r#"\w*https://tidal.com/track/(?P<trak>[0-9A-Za-z]+)[\s|\?]*[.]*"#).unwrap(); //for matching tidal tracks from content

    let mut spotify_msg :String = String::new();
    let mut apple_msg :String = String::new();
    let mut tidal_msg :String = String::new();
    let mut spotify_usr :String = String::new();
    let mut hexpub :String = String::new();    
    let mut final_txt :String = String::new();
    let mut final_txt1 :String = String::new();
    let mut final_txt2 :String = String::new();


    
    //Other useful IDs/tracklist to be used in the content of the notes -- these
    let nostr_playing="npub1s3qtkfsygc2hv8vjykevmc4rrdl5l7pa88w7lfru8tjh47qspeasve0qfm";
    let harmonique = "npub1706ejtxm88npppmg64plh5uy5y00cdcnpptp0m3gjvhtk9s5up7qxylh8d";
    let harmonique_mark = "npub1w59j6r40vq764w7qxp5kjrwsm3dzn8qyn0hta856ljl96rt5887sa875tg";
    let wavlake = "npub1yfg0d955c2jrj2080ew7pa4xrtj7x7s7umt28wh0zurwmxgpyj9shwv6vg";
    let nostr_radio = "npub1w65mgf77dfnn9c2vylw8k0rjjvvc8cw60ttw44u2cf0608eyxtlsyt9ec3";
    let playlist_phil = "npub1upmh82f2vy9z3k3qwj8amx9lkkhkjjcv45y9yfyqzfj4jj5czz9qhsg8fq";
    let playlist_build = "npub1nxy4qpqnld6kmpphjykvx2lqwvxmuxluddwjamm4nc29ds3elyzsm5avr7";
    let intuitive_guy = "npub13pnmakf738yn6rv2ex9jgs7924renmderyp5d9rtztsr7ymxg3gqej06vw";
    let postscript = "\n\nThis note generated via NDK nostr:npub1ndkltu98tr2eupgv5y367cck7kgj96d56vzmz40mxa25v68evv0qv6ffdn & using search filters on nostr.band";


    // Printing events only the reference and npub 
    if events.len()>0 
    {   let mut i = 0;
        let mut temp_event: &Event = &events[0];
        let mut find_str :String = String::new();
        let mut contents :String = String::new();
        i = 0;
        while i < events.len()
        {   temp_event = &events[i];
            find_str = temp_event.as_json(); 
            
            //Find the track(s) in the json along with the recommending user pubkey
            let matches = pubkey_re.captures(&find_str).unwrap();
            hexpub = XOnlyPublicKey::from_str(matches.get(1).unwrap().as_str()).unwrap().to_bech32()?;
            
            //i.e. do not read the note if this is from your ID - ie prevent loops where your previous day compilation notes get incorporated into the new days 
            if hexpub != bech32_pubkey
            {   //Extract contents from the JSON string - this is done through Regex; JSON parsing is too advanced for me
                contents = content_re.captures(&find_str).unwrap().get(1).unwrap().as_str().to_string();
                println!("Contents are:{}",contents);
                
                for trackname in spotify_re.captures_iter(&contents)
                {   spotify_msg.push_str("\nhttps://open.spotify.com/track/");
                    spotify_msg.push_str(&trackname["trak"]);
                    if spotify_usr.contains(&hexpub) //If this user is already in the recos list,do not add them again
                    {   println!("Sorry!Already present");}
                    else
                    {  spotify_usr.push_str("\n nostr:");
                       spotify_usr.push_str(hexpub.as_str());
                    }
                }
                
                for trackname in apple_re.captures_iter(&contents)
                {   apple_msg.push_str("\nhttps://music.apple.com/");
                    apple_msg.push_str(&trackname["trak"]);
                    apple_msg.push_str(" -- nostr:");
                    apple_msg.push_str(hexpub.as_str()); 
                }
                
                for trackname in tidal_re.captures_iter(&contents)
                {   tidal_msg.push_str("\nhttps://tidal.com/track/");
                    tidal_msg.push_str(&trackname["trak"]);
                    tidal_msg.push_str(" -- nostr:");
                    tidal_msg.push_str(hexpub.as_str()); 
                }
            }
            i = i + 1;
        }
    }
    
    client.disconnect_relay("wss://relay.nostr.band").await.unwrap();
    
    //Add relays you want to post the compiled list of tracks to, specifically as nostr.band is a read-only relay
    client.add_relay("wss://relay.damus.io", None).await?;
    client.add_relay("wss://nos.lol", None).await?;
    client.add_relay("wss://relay.nostr.bg", None).await?;
    
    client.connect().await;
    
    final_txt.push_str("#Music recos on Spotify from #Nostr on ");
    final_txt.push_str(since_time.format("%e-%b-%Y").to_string().as_str());
    final_txt.push_str("\n");
    final_txt.push_str(&spotify_msg);
    final_txt.push_str("\n\n Recos from:");
    final_txt.push_str(&spotify_usr);
    
    //Other recommendations: 
    final_txt.push_str("\n\nFor more music check nostrplaying nostr:");
    final_txt.push_str(nostr_playing);
    final_txt.push_str(" or Harmonique nostr:");
    final_txt.push_str(harmonique);
    final_txt.push_str(" by nostr:");
    final_txt.push_str(harmonique_mark);
    final_txt.push_str(postscript);

    println!("Spotify Message:{}",final_txt);

    //Publish a note with list of Spotify Tracks
    let e1 = client.publish_text_note(final_txt, &[]).await?;
    println!("{:#?}", e1);
    

    final_txt1.push_str("#Music recos on Apple Music from #Nostr on ");
    final_txt1.push_str(since_time.format("%e-%b-%Y").to_string().as_str());
    final_txt1.push_str("\n");
    final_txt1.push_str(&apple_msg);

    //Other recommendations: 
    final_txt1.push_str("\n\nFor more music check https://nostradio.fyi/ by nostr:");
    final_txt1.push_str(nostr_radio);
    final_txt1.push_str(" or Wavlake nostr:");
    final_txt1.push_str(wavlake);
    final_txt1.push_str(postscript);
    
    println!("Apple Music Message:{}",final_txt1);
   
    //Publish a note with list of Apple Music tracks
    let e2 = client.publish_text_note(final_txt1, &[]).await?;
    println!("{:#?}", e2);

    final_txt2.push_str("#Music recos on Tidal from #Nostr as on ");
    final_txt2.push_str(since_time.format("%e-%b-%Y").to_string().as_str());
    final_txt2.push_str("\n");
    final_txt2.push_str(&tidal_msg);
    
    //Other Recommendations
    final_txt2.push_str("\n\nNostr Tidal playlist https://tidal.com/playlist/7b1cde20-a28b-4859-9cce-ee4ec3adba3c by nostr:");
    final_txt2.push_str(playlist_phil);
    final_txt2.push_str("\nNostr Vibes playlist https://tidal.com/playlist/14382965-aacf-44f8-bb4d-a02ec7e41993 by nostr:");
    final_txt2.push_str(playlist_build);
    final_txt2.push_str("\nFollow Intuitive Guy nostr:");
    final_txt2.push_str(intuitive_guy);
    final_txt2.push_str(" for music sessions on NostrNests");
    final_txt2.push_str(postscript);

    println!("Tidal track is {}",final_txt2);
    
    //Publish a note with list of Tidal tracks. 
    //Note : a different string final_txt, txt_1, txt_2 is necessary for each music streamer as publish_text_note is async function  
    //& hence clearing the same string to be reused will not work as it would be trying to clear while it is still being used in previous publish_text_note 
    let e3 = client.publish_text_note(final_txt2, &[]).await?;
    println!("{:#?}", e3);

    //Disconnect from all relays before exit
    client.disconnect().await?;

    Ok(())
}