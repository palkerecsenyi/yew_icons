use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_tl (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-tl" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "tl-a" > < path fill - opacity = ".7" d = "M0 0h682.7v512H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "scale(.9375)" > < path fill = "#cb000f" d = "M0 0h1031.2v512H0z" /> < path fill = "#f8c00c" d = "M0 0c3.2 0 512 256.7 512 256.7L0 512V0z" /> < path d = "M0 0c2.1 0 340.6 256.7 340.6 256.7L0 512V0z" /> < path fill = "#fff" d = "M187.7 298.2 127 284.7l-31 52.8-5-59.7-60.7-13.3 54.9-24.9-3.3-59.3 40.2 43.4 55.4-25.3-28.9 54 39.2 45.8z" /> </ g > </ svg > } }