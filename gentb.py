
name=["班卓琴","baseattack","贝斯","bd","铃铛（钟琴）","“芯片”（方波)","牛铃","迪吉里杜管","长笛","吉他","harp2","竖琴/钢琴","击鼓沿" ,"管钟" ,"“铁木琴”（颤音琴）" ,"“扣弦”（电钢琴）" ,"小军鼓" ,"木琴" ]
block=[ "干草块" ,"我也不知道啥" ,"木质" ,"依然不知道啥" ,"金块" ,"绿宝石块" ,"灵魂沙" ,"南瓜" ,"黏土块" ,"羊毛" ,"不知道啥" ,"其他方块" ,"玻璃" ,"浮冰" ,"铁块" ,"荧石" ,"沙子" ,"骨块" ]
note = [ "F#","G","G#","A","A#","B","C","C#","D","D#","E","F","F#","G","G#","A","A#","B","C","C#","D","D#","E","F","F#"]


for o in range(18):
    print("<tr><th>%s</th><th>%s</th>"%(name[o],block[o]))
    for i in range(25):
        print("<th><input type='button' onclick='pc.play(%d,%d)' value='%d %s'/></th>"%(o,i,i,note[i] ))
    print("</tr>")