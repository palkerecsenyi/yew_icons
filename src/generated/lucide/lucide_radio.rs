use crate :: IconProps ; # [inline (never)] pub fn lucide_radio (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "12" cy = "12" r = "2" /> < path d = "M4.93 19.07a10 10 0 0 1 0-14.14" /> < path d = "M7.76 16.24a6 6 0 0 1-1.3-1.95 6 6 0 0 1 0-4.59 6 6 0 0 1 1.3-1.95" /> < path d = "M16.24 7.76a6 6 0 0 1 1.3 2 6 6 0 0 1 0 4.59 6 6 0 0 1-1.3 1.95" /> < path d = "M19.07 4.93a10 10 0 0 1 0 14.14" /> </ svg > } }