use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hands (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M330.8 242.3L223.1 209.1C210.3 205.2 197 212.3 193.1 224.9C189.2 237.6 196.3 251 208.9 254.9L256 272H56.9c-11.61 0-22.25 7.844-24.44 19.24C29.51 306.6 41.19 320 56 320h128C188.4 320 192 323.6 192 328S188.4 336 184 336H24.9c-11.61 0-22.25 7.844-24.44 19.24C-2.49 370.6 9.193 384 24 384h160C188.4 384 192 387.6 192 392S188.4 400 184 400H56.9c-11.61 0-22.25 7.844-24.44 19.24C29.51 434.6 41.19 448 56 448h128C188.4 448 192 451.6 192 456S188.4 464 184 464H88.9c-11.61 0-22.25 7.844-24.44 19.24C61.51 498.6 73.19 512 88 512h208c66.28 0 120-53.73 120-120v-32.03C416 306.6 381.1 259.4 330.8 242.3zM197.1 179.5c5.986-2.148 12.32-3.482 18.98-3.482c5.508 0 10.99 .8105 16.5 2.471l16.11 4.975L227.7 117.2C224.2 106.2 213.6 98.39 202 99.74c-15.51 1.807-24.79 16.99-20.33 31.11L197.1 179.5zM487.1 144.5c-13.27 .0977-23.95 10.91-23.86 24.16l-2.082 50.04l-59.98-189.8c-3.496-11.07-14.18-18.86-25.71-17.51c-15.51 1.807-24.79 16.99-20.33 31.11l38.56 122.1c1.332 4.213-1.004 8.707-5.219 10.04c-4.213 1.332-8.707-1.004-10.04-5.217l-47.93-151.7c-3.496-11.07-14.18-18.86-25.71-17.51c-15.51 1.807-24.79 16.99-20.33 31.11l43.37 137.8c1.33 4.213-1.006 8.707-5.219 10.04c-4.213 1.332-8.707-1.004-10.04-5.217l-33.46-106.4C275.6 56.39 264.9 48.6 253.4 49.94c-15.51 1.807-24.79 16.99-20.33 31.11l34.15 108.1l73.7 22.76C404.1 233.3 448 292.8 448 359.9v27.91c38.27-21.17 63.28-61.24 64-106.7V168.4C511.8 155.1 500.3 144.5 487.1 144.5z" /></ svg > } }