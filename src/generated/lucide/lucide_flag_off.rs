use crate :: IconProps ; # [inline (never)] pub fn lucide_flag_off (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M8 2c3 0 5 2 8 2s4-1 4-1v11" /> < path d = "M4 22V4" /> < path d = "M4 15s1-1 4-1 5 2 8 2" /> < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" /> </ svg > } }