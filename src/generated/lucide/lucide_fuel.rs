use crate :: IconProps ; # [inline (never)] pub fn lucide_fuel (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "3" y1 = "22" x2 = "15" y2 = "22" /> < line x1 = "4" y1 = "9" x2 = "14" y2 = "9" /> < path d = "M14 22V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v18" /> < path d = "M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V9.83a2 2 0 0 0-.59-1.42L18 5" /> </ svg > } }