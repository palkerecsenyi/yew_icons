use crate :: IconProps ; # [inline (never)] pub fn lucide_align_horizontal_space_between (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "3" y = "5" width = "6" height = "14" rx = "2" /> < rect x = "15" y = "7" width = "6" height = "10" rx = "2" /> < path d = "M3 2v20" /> < path d = "M21 2v20" /> </ svg > } }