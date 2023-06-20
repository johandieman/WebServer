import whisper
import io
from moviepy.editor import VideoFileClip, CompositeVideoClip, TextClip
from moviepy.video.tools.subtitles import SubtitlesClip

class Video:
        def __init__(self, arr):
                input_file = io.BytesIO(arr)
                model = whisper.load_model("base")
                audio = whisper.load_audio("test.mp3")
                result = model.transcribe(audio)
                print(result['segments'])

                generator = lambda txt: TextClip(txt, font='Arial', fontsize=24, color='white')
                subs = [((0, 4), 'subs1'),
                        ((4, 9), 'subs2'),
                        ((9, 12), 'subs3'),
                        ((12, 16), 'subs4')]
                subtitles = SubtitlesClip(subs, generator)
                video = VideoFileClip(input_file)
                self.result = CompositeVideoClip([video, subtitles.set_pos(('center','bottom'))])
                self.result.write_videofile("output.mp4", fps=video.fps, temp_audiofile="temp-audio.m4a", remove_temp=True, codec="libx264", audio_codec="aac")
                
        def getResult(self):
                return self.result;

