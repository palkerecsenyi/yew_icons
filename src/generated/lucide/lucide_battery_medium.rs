use crate :: IconProps ; # [inline (never)] pub fn lucide_battery_medium (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "2" y = "7" width = "16" height = "10" rx = "2" ry = "2" /> < line x1 = "22" x2 = "22" y1 = "11" y2 = "13" /> < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" /> < line x1 = "10" x2 = "10" y1 = "11" y2 = "13" /> </ svg > } }