use crate :: IconProps ; # [inline (never)] pub fn simple_icons_hackthebox (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.9959.0008a1.1187 1.1187 0 00-.057.002.8993.8993 0 00-.2358.0498.9067.9067 0 00-.1652.079L1.9357 5.675a.889.889 0 00-.4444.7699c0 .006.0004.0128.0006.0192-.0002.007 0 .014 0 .0212V17.556a.889.889 0 00.469.7837l9.5983 5.5416c.018.0102.036.0197.054.0287v.002a.8568.8568 0 00.083.0348c0 .001.01.003.012.004.028.01.056.0177.085.0245.01.001.011.003.016.004.028.006.057.0112.086.0146 0 .0005.01.0009.014.001.03.003.061.005.091.005s.061-.002.091-.005c0-.0005.01-.0009.014-.001a.6831.6831 0 00.086-.0146c.01-.001.011-.002.016-.004a.9404.9404 0 00.085-.0245c0-.001.01-.003.012-.004a.8818.8818 0 00.083-.0347v-.002a1.086 1.086 0 00.054-.0287l9.5986-5.5416a.889.889 0 00.4689-.7837V6.4786c0-.009-.0006-.0172-.0008-.0258h.0003v-.008a.8886.8886 0 00-.3117-.6755c-.01-.008-.019-.0162-.029-.0241 0-.002-.01-.005-.01-.007a.8988.8988 0 00-.1074-.0705L12.4533.1267a.8872.8872 0 00-.4646-.1266zm.01 2.2523c.072 0 .1443.0187.209.056l6.5366 3.774c.2789.161.2789.5633 0 .7243l-6.5367 3.774a.4182.4182 0 01-.4182 0L5.26 6.8074c-.2788-.1609-.2789-.5633 0-.7243l6.5368-3.774a.4193.4193 0 01.209-.056zm-8.0801 6.458a.4145.4145 0 01.215.0565l6.524 3.7666a.417.417 0 01.2086.3612v7.5326c0 .3212-.3477.522-.626.3613l-6.5237-3.7666a.4172.4172 0 01-.2086-.3613V9.1288c0-.2408.1955-.414.4107-.4177zm16.1599 0c.215.004.4107.1768.4107.4177v7.5325c0 .149-.08.2868-.2087.3614l-6.5239 3.7666c-.278.1606-.6258-.0401-.6258-.3614v-7.5325c0-.149.08-.2867.2086-.3613l6.5238-3.7666a.415.415 0 01.2152-.0565z" /></ svg > } }