use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_elevator (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M79 96h130c5.967 0 11.37-3.402 13.75-8.662c2.385-5.262 1.299-11.39-2.754-15.59l-65-67.34c-5.684-5.881-16.31-5.881-21.99 0l-65 67.34C63.95 75.95 62.87 82.08 65.25 87.34C67.63 92.6 73.03 96 79 96zM357 91.59c5.686 5.881 16.31 5.881 21.99 0l65-67.34c4.053-4.199 5.137-10.32 2.754-15.59C444.4 3.402 438.1 0 433 0h-130c-5.967 0-11.37 3.402-13.75 8.662c-2.385 5.262-1.301 11.39 2.752 15.59L357 91.59zM448 128H64c-35.35 0-64 28.65-64 63.1v255.1C0 483.3 28.65 512 64 512h384c35.35 0 64-28.65 64-63.1V192C512 156.7 483.3 128 448 128zM352 224C378.5 224.1 400 245.5 400 272c0 26.46-21.47 47.9-48 48C325.5 319.9 304 298.5 304 272C304 245.5 325.5 224.1 352 224zM160 224C186.5 224.1 208 245.5 208 272c0 26.46-21.47 47.9-48 48C133.5 319.9 112 298.5 112 272C112 245.5 133.5 224.1 160 224zM240 448h-160v-48C80 373.5 101.5 352 128 352h64c26.51 0 48 21.49 48 48V448zM432 448h-160v-48c0-26.51 21.49-48 48-48h64c26.51 0 48 21.49 48 48V448z" /></ svg > } }