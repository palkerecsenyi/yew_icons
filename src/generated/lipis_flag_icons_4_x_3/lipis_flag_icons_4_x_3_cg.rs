use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_cg (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-cg" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "cg-a" > < path fill - opacity = ".7" d = "M-79.5 32h640v480h-640z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "1pt" transform = "translate(79.5 -32)" > < path fill = "#ff0" d = "M-119.5 32h720v480h-720z" /> < path fill = "#00ca00" d = "M-119.5 32v480l480-480h-480z" /> < path fill = "red" d = "M120.5 512h480V32l-480 480z" /> </ g > </ svg > } }