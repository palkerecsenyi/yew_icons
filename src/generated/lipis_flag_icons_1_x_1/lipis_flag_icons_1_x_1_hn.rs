use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_hn (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-hn" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#18c3df" d = "M0 0h512v512H0z" /> < path fill = "#fff" d = "M0 170.7h512v170.6H0z" /> < g id = "c" fill = "#18c3df" transform = "translate(256 256) scale(28.44446)" > < g id = "b" > < path id = "a" d = "m0-1-.3 1 .5.1z" /> < use href = "#a" width = "100%" height = "100%" transform = "scale(-1 1)" /> </ g > < use href = "#b" width = "100%" height = "100%" transform = "rotate(72)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(-72)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(144)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(-144)" /> </ g > < use href = "#c" width = "100%" height = "100%" transform = "translate(142.2 -45.5)" /> < use href = "#c" width = "100%" height = "100%" transform = "translate(142.2 39.8)" /> < use href = "#c" width = "100%" height = "100%" transform = "translate(-142.2 -45.5)" /> < use href = "#c" width = "100%" height = "100%" transform = "translate(-142.2 39.8)" /> </ svg > } }