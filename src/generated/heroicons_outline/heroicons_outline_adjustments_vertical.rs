use crate :: IconProps ; # [inline (never)] pub fn heroicons_outline_adjustments_vertical (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/tailwindlabs/heroicons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "none" viewBox = "0 0 24 24" stroke - width = "1.5" stroke = "currentColor" aria - hidden = "true" > if let Some (title) = title . clone () { < title > { title } </ title > } < path stroke - linecap = "round" stroke - linejoin = "round" d = "M6 13.5V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 9.75V10.5" /> </ svg > } }