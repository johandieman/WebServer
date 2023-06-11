import whisper

def handler():
    # model = whisper.load_model("base",)

    # # load audio and pad/trim it to fit 30 seconds
    # audio = whisper.load_audio("test.mp3")
    # audio = whisper.pad_or_trim(audio)

    # # make log-Mel spectrogram and move to the same device as the model
    # mel = whisper.log_mel_spectrogram(audio).to(model.device)

    # # detect the spoken language
    # _, probs = model.detect_language(mel)
    # print(f"Detected language: {max(probs, key=probs.get)}")

    # # decode the audio
    # options = whisper.DecodingOptions(fp16 = False,)
    # result = whisper.transcribe(model, mel, options)

    model = whisper.load_model("base")
    audio = whisper.load_audio("test.mp3")
    result = model.transcribe(audio)
    # print the recognized text
    print(result['segments'])


handler()