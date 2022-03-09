# RusTTS

`RusTTS` is an unofficial [ `Coqui TTS` ](https://github.com/coqui-ai/tts) implementation.

Currently, only the `YourTTS` for [ `TTS` & `VC` ] has been implemented.

So, feel free to contribute us to make it better.

## Milestones

* [x] YourTTS + TTS
* [x] YourTTS + VC
* [ ] Attach LICENSE for sample data
* [ ] Non-English Characters
* [ ] [`Arpabet`](https://github.com/echelon/arpabet.rs) Support
* [ ] SSML support
    - Note: `Coqui TTS` has no `SSML support` yet, so we can support both.

## Import from Pretrained Models

Currently, only the `YourTTS` for [ `TTS` & `VC` ] has been implemented.

1. TODO: Show the `diff` of Coqui TTS source code.
2. Run the code: `./assets/vits/converter.ipynb`
    - The original code is derived from `VITS` examples.
    - TODO: Store the models in the Cloud Storage, not in git-managed.

## Examples

### TTS

```bash
cargo run --example tts -- "Hello Mr. my yesterday"
```

### Voice Conversion

```bash
cargo run --example vc
```
