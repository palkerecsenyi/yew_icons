use crate :: IconProps ; # [inline (never)] pub fn feather_archive (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polyline points = "21 8 21 21 3 21 3 8" /> < rect x = "1" y = "3" width = "22" height = "5" /> < line x1 = "10" y1 = "12" x2 = "14" y2 = "12" /> </ svg > } }