use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_so (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-so" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "so-a" > < path fill - opacity = ".7" d = "M-85.3 0h682.6v512H-85.3z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(80) scale(.9375)" > < path fill = "#40a6ff" d = "M-128 0h768v512h-768z" /> < path fill = "#fff" d = "M336.5 381.2 254 327.7l-82.1 54 30.5-87.7-82-54.2L222 239l31.4-87.5 32.1 87.3 101.4.1-81.5 54.7 31.2 87.6z" /> </ g > </ svg > } }