use crate :: IconProps ; # [inline (never)] pub fn lucide_languages (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m5 8 6 6" /> < path d = "m4 14 6-6 2-3" /> < path d = "M2 5h12" /> < path d = "M7 2h1" /> < path d = "m22 22-5-10-5 10" /> < path d = "M14 18h6" /> </ svg > } }