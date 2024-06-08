// Code: examples/main

use elevenlabs_rs::client::{ElevenLabsClient, Result};
use elevenlabs_rs::endpoints::pronunciation::*;
use elevenlabs_rs::endpoints::tts::*;
use elevenlabs_rs::utils::play;

#[tokio::main]
async fn main() -> Result<()> {
    let body = AddFromFileBody::new(
        "examples/pronunciation_dictionaries/dictionary.pls",
        "example",
    );
    let client = ElevenLabsClient::new()?;
    let resp = client.hit(AddFromFile::new(body)).await?;
    let mut dictionary_id = resp.id();
    let mut version_id = resp.version_id();

    let mut pls_bytes = client
        .hit(DownloadVersionByID::new(dictionary_id, version_id))
        .await?;
    let mut current_dictionary_state = unsafe { std::str::from_utf8_unchecked(&pls_bytes) };
    println!("--- Initial dictionary ---\n");
    println!("{}\n", current_dictionary_state);

    let txt = "I'm using a TTS model via the ElevenLabs' API to say tomato and Tomato.";
    let model = "eleven_turbo_v2";
    let rachel_voice_id = "21m00Tcm4TlvDq8ikWAM";

    let mut tts_body = TextToSpeechBody::new(txt, model);
    let mut tts_endpoint = TextToSpeech::new(rachel_voice_id, tts_body);
    let speech_without_dict = client.hit(tts_endpoint).await?;

    tts_body = TextToSpeechBody::new(txt, model).with_pronunciation_dict(dictionary_id, version_id);
    tts_endpoint = TextToSpeech::new(rachel_voice_id, tts_body);
    let speech_with_dict = client.hit(tts_endpoint).await?;

    let rules = vec![
        Rule::new_alias("TTS", "text to speech"),
        Rule::new_alias("API", "application programming interface"),
        Rule::new_phoneme("via", "/ˈvaɪə/", "ipa"),
    ];
    let mut resp = client.hit(AddRules::new(dictionary_id, rules)).await?;
    dictionary_id = resp.id();
    version_id = resp.version_id();

    pls_bytes = client
        .hit(DownloadVersionByID::new(dictionary_id, version_id))
        .await?;
    current_dictionary_state = unsafe { std::str::from_utf8_unchecked(&pls_bytes) };
    println!("--- Dictionary with added rules ---\n");
    println!("{}\n", current_dictionary_state);

    tts_body = TextToSpeechBody::new(txt, model).with_pronunciation_dict(dictionary_id, version_id);
    tts_endpoint = TextToSpeech::new(rachel_voice_id, tts_body);
    let speech_with_added_rules = client.hit(tts_endpoint).await?;

    let remove_rules = vec!["TTS", "tomato", "Tomato"];
    resp = client
        .hit(RemoveRules::new(dictionary_id, remove_rules))
        .await?;
    dictionary_id = resp.id();
    version_id = resp.version_id();

    pls_bytes = client
        .hit(DownloadVersionByID::new(dictionary_id, version_id))
        .await?;
    current_dictionary_state = unsafe { std::str::from_utf8_unchecked(&pls_bytes) };
    println!("--- Dictionary with removed rules ---\n");
    println!("{}\n", current_dictionary_state);

    tts_body = TextToSpeechBody::new(txt, model).with_pronunciation_dict(dictionary_id, version_id);
    tts_endpoint = TextToSpeech::new(rachel_voice_id, tts_body);
    let speech_with_removed_rules = client.hit(tts_endpoint).await?;

    play(speech_without_dict)?;
    play(speech_with_dict)?;
    play(speech_with_added_rules)?;
    play(speech_with_removed_rules)?;

    Ok(())
}
