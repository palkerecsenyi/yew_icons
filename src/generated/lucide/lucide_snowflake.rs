use crate :: IconProps ; # [inline (never)] pub fn lucide_snowflake (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> < line x1 = "12" y1 = "2" x2 = "12" y2 = "22" /> < path d = "m20 16-4-4 4-4" /> < path d = "m4 8 4 4-4 4" /> < path d = "m16 4-4 4-4-4" /> < path d = "m8 20 4-4 4 4" /> </ svg > } }