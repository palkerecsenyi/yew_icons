use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_shield_virus (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M288 255.1c-8.836 0-16 7.162-16 16c0 8.836 7.164 15.1 16 15.1s16-7.163 16-15.1C304 263.2 296.8 255.1 288 255.1zM224 191.1c-8.836 0-16 7.162-16 16c0 8.836 7.164 16 15.1 16s16-7.164 16-16C240 199.2 232.8 191.1 224 191.1zM466.5 83.68l-192-80.01C269.6 1.641 261.3 0 256.1 0C250.7 0 242.5 1.641 237.6 3.672l-192 80.01C27.69 91.07 16 108.6 16 127.1C16 385.2 205.2 512 255.9 512c52.02 0 240.1-128.2 240.1-384C496 108.6 484.3 91.07 466.5 83.68zM384 255.1h-12.12c-19.29 0-32.06 15.78-32.06 32.23c0 7.862 2.918 15.88 9.436 22.4l8.576 8.576c3.125 3.125 4.688 7.218 4.688 11.31c0 8.527-6.865 15.1-16 15.1c-4.094 0-8.188-1.562-11.31-4.688l-8.576-8.576c-6.519-6.519-14.53-9.436-22.4-9.436c-16.45 0-32.23 12.77-32.23 32.06v12.12c0 8.844-7.156 16-16 16s-16-7.156-16-16v-12.12c0-19.29-15.78-32.06-32.23-32.06c-7.862 0-15.87 2.917-22.39 9.436l-8.576 8.576c-3.125 3.125-7.219 4.688-11.31 4.688c-9.139 0-16-7.473-16-15.1c0-4.094 1.562-8.187 4.688-11.31l8.576-8.576c6.519-6.519 9.436-14.53 9.436-22.4c0-16.45-12.77-32.23-32.06-32.23H128c-8.844 0-16-7.156-16-16s7.156-16 16-16h12.12c19.29 0 32.06-15.78 32.06-32.23c0-7.862-2.918-15.88-9.436-22.4L154.2 160.8C151 157.7 149.5 153.6 149.5 149.5c0-8.527 6.865-15.1 16-15.1c4.094 0 8.188 1.562 11.31 4.688L185.4 146.7C191.9 153.3 199.9 156.2 207.8 156.2c16.45 0 32.23-12.77 32.23-32.07V111.1c0-8.844 7.156-16 16-16s16 7.156 16 16v12.12c0 19.29 15.78 32.07 32.23 32.07c7.862 0 15.88-2.917 22.4-9.436l8.576-8.577c3.125-3.125 7.219-4.688 11.31-4.688c9.139 0 16 7.473 16 15.1c0 4.094-1.562 8.187-4.688 11.31l-8.576 8.577c-6.519 6.519-9.436 14.53-9.436 22.4c0 16.45 12.77 32.23 32.06 32.23h12.12c8.844 0 16 7.156 16 16S392.8 255.1 384 255.1z" /></ svg > } }