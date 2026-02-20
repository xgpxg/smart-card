use aha::models::WhichModel::Qwen3ASR0_6B;
use aha::models::qwen3_asr::generate::Qwen3AsrGenerateModel;
use aha::models::{GenerateModel, ModelInstance, WhichModel};
use aha_openai_dive::v1::resources::chat::{
    AudioUrlType, ChatCompletionParameters, ChatMessage, ChatMessageAudioContentPart,
    ChatMessageContent, ChatMessageContentPart,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

static MODEL: OnceLock<Arc<Mutex<Qwen3AsrGenerateModel>>> = OnceLock::new();

pub async fn load_model(model_save_path: &str) -> anyhow::Result<()> {
    aha::utils::download_model(Qwen3ASR0_6B.model_id(), model_save_path, 3).await?;

    let model_path = Path::new(model_save_path).join(Qwen3ASR0_6B.model_id());

    MODEL.get_or_init(|| {
        Arc::new(Mutex::new(
            Qwen3AsrGenerateModel::init(&model_path.display().to_string().as_str(), None, None)
                .unwrap(),
        ))
    });
    Ok(())
}
pub async fn run(file_path: &str) -> anyhow::Result<String> {
    let mut model = MODEL.get().unwrap().lock().unwrap();
    let mut parameters = ChatCompletionParameters::default();
    parameters.messages = vec![ChatMessage::User {
        content: ChatMessageContent::ContentPart(vec![ChatMessageContentPart::Audio(
            ChatMessageAudioContentPart {
                r#type: "audio".to_string(),
                audio_url: AudioUrlType {
                    url: format!("file://{}", file_path),
                },
            },
        )]),
        name: None,
    }];

    let response = model.generate(parameters)?;

    let json = serde_json::to_value(response)?;

    let content = json["choices"][0]["message"]["content"].as_str().unwrap();

    // 替换从开头到<asr_text>的部分
    let content = content.split("<asr_text>").collect::<Vec<&str>>()[1];
    println!("{}", content);
    Ok(content.to_string())
}
