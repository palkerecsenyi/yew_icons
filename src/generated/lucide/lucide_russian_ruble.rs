use crate :: IconProps ; # [inline (never)] pub fn lucide_russian_ruble (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M14 11c5.333 0 5.333-8 0-8" /> < path d = "M6 11h8" /> < path d = "M6 15h8" /> < path d = "M9 21V3" /> < path d = "M9 3h5" /> </ svg > } }