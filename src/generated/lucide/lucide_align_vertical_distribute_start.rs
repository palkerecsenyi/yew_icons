use crate :: IconProps ; # [inline (never)] pub fn lucide_align_vertical_distribute_start (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "5" y = "14" width = "14" height = "6" rx = "2" /> < rect x = "7" y = "4" width = "10" height = "6" rx = "2" /> < path d = "M2 14h20" /> < path d = "M2 4h20" /> </ svg > } }