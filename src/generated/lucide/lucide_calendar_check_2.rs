use crate :: IconProps ; # [inline (never)] pub fn lucide_calendar_check_2 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M21 14V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" /> < line x1 = "16" y1 = "2" x2 = "16" y2 = "6" /> < line x1 = "8" y1 = "2" x2 = "8" y2 = "6" /> < line x1 = "3" y1 = "10" x2 = "21" y2 = "10" /> < path d = "m16 20 2 2 4-4" /> </ svg > } }