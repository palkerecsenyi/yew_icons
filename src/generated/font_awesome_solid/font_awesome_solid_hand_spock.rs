use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hand_spock (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M543.6 128.6c0-8.999-6.115-32.58-31.68-32.58c-14.1 0-27.02 9.324-30.92 23.56l-34.36 125.1c-1.682 6.16-7.275 10.43-13.66 10.43c-7.981 0-14.16-6.518-14.16-14.13c0-.9844 .1034-1.987 .3197-2.996l35.71-166.6c.5233-2.442 .7779-4.911 .7779-7.362c0-13.89-9.695-32.86-31.7-32.86c-14.79 0-28.12 10.26-31.34 25.29l-37.77 176.2c-2.807 13.1-14.38 22.46-27.77 22.46c-13.04 0-24.4-8.871-27.56-21.52l-52.11-208.5C243.6 11.2 230.5-.0013 215.6-.0013c-26.71 0-31.78 25.71-31.78 31.98c0 2.569 .3112 5.18 .9617 7.786l50.55 202.2c.2326 .9301 .3431 1.856 .3431 2.764c0 6.051-4.911 11.27-11.3 11.27c-4.896 0-9.234-3.154-10.74-7.812L166.9 103.9C162.4 89.1 149.5 80.02 135.5 80.02c-15.68 0-31.63 12.83-31.63 31.97c0 3.273 .5059 6.602 1.57 9.884l69.93 215.7c.2903 .8949 .4239 1.766 .4239 2.598c0 4.521-3.94 7.915-8.119 7.915c-1.928 0-3.906-.7219-5.573-2.388L101.7 285.3c-8.336-8.336-19.63-12.87-30.81-12.87c-23.56 0-39.07 19.69-39.07 39.55c0 10.23 3.906 20.47 11.72 28.28l122.5 122.5C197.6 494.3 240.3 512 284.9 512h50.98c23.5 0 108.4-14.57 132.5-103l73.96-271.2C543.2 134.8 543.6 131.7 543.6 128.6z" /></ svg > } }