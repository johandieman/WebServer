import whisper
import io
from moviepy.editor import VideoFileClip, CompositeVideoClip, TextClip
from moviepy.video.tools.subtitles import SubtitlesClip

def RunModelAndAlterVideo(arr):

    input_file = io.BytesIO(arr)

    model = whisper.load_model("base")
    audio = whisper.load_audio("test.mp3")
    result = model.transcribe(audio)
    # print the recognized text
    print(result['segments'])

    generator = lambda txt: TextClip(txt, font='Arial', fontsize=24, color='white')
    subs = [((0, 4), 'subs1'),
            ((4, 9), 'subs2'),
            ((9, 12), 'subs3'),
            ((12, 16), 'subs4')]
    subtitles = SubtitlesClip(subs, generator)
    video = VideoFileClip(input_file)
    result = CompositeVideoClip([video, subtitles.set_pos(('center','bottom'))])
    result.write_videofile("output.mp4", fps=video.fps, temp_audiofile="temp-audio.m4a", remove_temp=True, codec="libx264", audio_codec="aac")


