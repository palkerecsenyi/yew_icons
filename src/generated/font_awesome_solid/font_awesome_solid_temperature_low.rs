use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_temperature_low (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M160 322.9V304C160 295.3 152.8 288 144 288S128 295.3 128 304v18.88C109.4 329.5 96 347.1 96 368C96 394.5 117.5 416 144 416S192 394.5 192 368C192 347.1 178.6 329.5 160 322.9zM256 112c0-61.88-50.13-112-112-112s-112 50.13-112 112v166.5c-19.75 24.75-32 55.5-32 89.5c0 79.5 64.5 144 144 144s144-64.5 144-144c0-33.1-12.25-64.88-32-89.5V112zM144 448c-44.13 0-80-35.88-80-80c0-25.5 12.25-48.88 32-63.75v-192.3c0-26.5 21.5-48 48-48s48 21.5 48 48v192.1c19.75 14.75 32 38.38 32 63.88C224 412.1 188.1 448 144 448zM416 0c-52.88 0-96 43.13-96 96s43.13 96 96 96s96-43.13 96-96S468.9 0 416 0zM416 128c-17.75 0-32-14.25-32-32s14.25-32 32-32s32 14.25 32 32S433.8 128 416 128z" /></ svg > } }