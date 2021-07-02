# Adding more sounds

To add a new SFX called new_sound, you need to:
1. Add it in this directory with name `new_sound.wav`
2. On `audio.rs`, in the enum `SFX`, create a new variant called `NewSound`.
3. Now, anywhere in code, just use `SFX::NewSound.play()`!