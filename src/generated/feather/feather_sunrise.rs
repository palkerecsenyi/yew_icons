use crate :: IconProps ; # [inline (never)] pub fn feather_sunrise (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17 18a5 5 0 0 0-10 0" /> < line x1 = "12" y1 = "2" x2 = "12" y2 = "9" /> < line x1 = "4.22" y1 = "10.22" x2 = "5.64" y2 = "11.64" /> < line x1 = "1" y1 = "18" x2 = "3" y2 = "18" /> < line x1 = "21" y1 = "18" x2 = "23" y2 = "18" /> < line x1 = "18.36" y1 = "11.64" x2 = "19.78" y2 = "10.22" /> < line x1 = "23" y1 = "22" x2 = "1" y2 = "22" /> < polyline points = "8 6 12 2 16 6" /> </ svg > } }