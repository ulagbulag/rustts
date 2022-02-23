# RusTTS

`RusTTS` is an unofficial [ `Coqui TTS` ](https://github.com/coqui-ai/tts) implementation.

Currently, only the `YourTTS` for `VC` has been implemented.

So, feel free to contribute us to make it better.

## Import from Pretrained Models

Currently, only the `YourTTS` for `VC` has been implemented.

```python
import os
import torch

# Create a model directory
os.makedirs('assets/', exist_ok=True)

# Load the pretrained models & variables
# NOTE: use the variables from `YourTTS_zeroshot_VC_demo.ipynb`
# NOTE: https://github.com/edresson/yourtts#colab-demos
SE_speaker_manager = ...
model = ...

driving_spec = ...
y_lengths = ...
driving_emb = ...
target_emb = ...

# Convert the pretrained model of speaker_encoder
# NOTE: Please mark ResNetSpeakerEncoder::l2_norm => true
model_speaker_encoder = torch.jit.trace(
    SE_speaker_manager.speaker_encoder,
    torch.randn(1, 129150),  # dummy
)
model_speaker_encoder.save('assets/speaker_encoder.pt')

# Convert the pretrained model of Vits (YourTTS)
class VoiceConversion(nn.Module):
    def __init__(self, model):
        super().__init__()
        self.model = model
        
    def forward(self, *args):
        return self.model.voice_conversion(*args)

model_voice_conversion = VoiceConversion(model)
model_voice_conversion = torch.jit.trace(
    model_voice_conversion,
    (driving_spec, y_lengths, driving_emb, target_emb,),  # maybe dummy
)
model_voice_conversion.save('assets/vits.pt')
```
