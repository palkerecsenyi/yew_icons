use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_martini_glass (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M502 57.63C523.3 36.38 508.3 0 478.3 0H33.72C3.711 0-11.29 36.38 9.962 57.63l214 214V448H175.1c-26.51 0-47.1 21.49-47.1 48c0 8.836 7.164 16 16 16h224c8.836 0 16-7.164 16-16c0-26.51-21.49-48-48-48h-47.1V271.6L502 57.63zM405.1 64l-64.01 64H170.9L106.9 64H405.1z" /></ svg > } }