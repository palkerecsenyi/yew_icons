use crate :: IconProps ; # [inline (never)] pub fn lucide_shuffle (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polyline points = "16 3 21 3 21 8" /> < line x1 = "4" y1 = "20" x2 = "21" y2 = "3" /> < polyline points = "21 16 21 21 16 21" /> < line x1 = "15" y1 = "15" x2 = "21" y2 = "21" /> < line x1 = "4" y1 = "4" x2 = "9" y2 = "9" /> </ svg > } }