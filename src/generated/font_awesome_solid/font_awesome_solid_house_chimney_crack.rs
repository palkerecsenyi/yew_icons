use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_house_chimney_crack (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M575.8 255.5C575.8 273.5 560.8 287.6 543.8 287.6H511.8L512.5 447.7C512.6 483.2 483.9 512 448.5 512H326.4L288 448L368.8 380.7C376.6 374.1 376.5 362.1 368.5 355.8L250.6 263.2C235.1 251.7 216.8 270.1 227.8 285.2L288 368L202.5 439.2C196.5 444.3 194.1 452.1 199.1 459.8L230.4 512H128.1C92.75 512 64.09 483.3 64.09 448V287.6H32.05C14.02 287.6 0 273.5 0 255.5C0 246.5 3.004 238.5 10.01 231.5L266.4 8.016C273.4 1.002 281.4 0 288.4 0C295.4 0 303.4 2.004 309.5 7.014L416 100.7V64C416 46.33 430.3 32 448 32H480C497.7 32 512 46.33 512 64V185L564.8 231.5C572.8 238.5 576.9 246.5 575.8 255.5L575.8 255.5z" /></ svg > } }