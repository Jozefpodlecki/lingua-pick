use whisper_rs::WhisperContextParameters;
use anyhow::Result;

pub struct TranscriptionService;

impl TranscriptionService {
    pub fn new() -> Result<Self> {

        let mut context_param = WhisperContextParameters::default();
        context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
            model_preset: whisper_rs::DtwModelPreset::BaseEn,
        };

        let custom_aheads = [
            (3, 1),
            (4, 2),
            (4, 3),
            (4, 7),
            (5, 1),
            (5, 2),
            (5, 4),
            (5, 6),
        ]
        .map(|(n_text_layer, n_head)| whisper_rs::DtwAhead {
            n_text_layer,
            n_head,
        });
        context_param.dtw_parameters.mode = whisper_rs::DtwMode::Custom {
            aheads: &custom_aheads,
        };

        Ok(Self{})
    }

    pub fn transcribe(&self, data: Vec<f32>) -> Result<()> {
        Ok(())
    }
}
