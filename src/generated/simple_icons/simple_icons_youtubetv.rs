use crate :: IconProps ; # [inline (never)] pub fn simple_icons_youtubetv (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M21.794 10.609h-.618v3.286h-.607v-3.286h-.617v-.504h1.842v.504zm.135-.504h.649l.202 1.703c.057.488.11.908.172 1.484h.026c.057-.576.114-.986.17-1.484l.208-1.703H24l-.623 3.784h-.82l-.628-3.784zm-15.758.38a.796.796 0 0 0-.555-.561c-.493-.13-2.465-.13-2.465-.13s-1.973 0-2.466.13a.787.787 0 0 0-.555.56C0 10.977 0 12 0 12s0 1.028.13 1.516a.794.794 0 0 0 .56.56c.488.13 2.46.13 2.46.13s1.973 0 2.466-.13a.806.806 0 0 0 .56-.555c.13-.493.13-1.516.13-1.516s-.005-1.028-.135-1.52zm-3.649 2.46v-1.89l1.64.945-1.64.945zm6.629.867a.652.652 0 0 1-.265-.395 2.703 2.703 0 0 1-.078-.706v-.4a2.383 2.383 0 0 1 .093-.716.724.724 0 0 1 .28-.394.943.943 0 0 1 .504-.125c.203 0 .369.042.493.125.13.098.229.233.27.394.062.234.094.472.083.716v.4c.005.239-.02.478-.083.711a.7.7 0 0 1-.27.395c-.124.083-.296.124-.508.124-.218-.005-.395-.046-.52-.13zm.7-.436c.037-.089.052-.24.052-.442v-.856a1.376 1.376 0 0 0-.052-.436.189.189 0 0 0-.181-.14.183.183 0 0 0-.177.14c-.041.14-.057.29-.052.436v.861c0 .203.016.353.052.442.02.083.094.14.182.135a.183.183 0 0 0 .176-.14zm8.88-.66v.14c0 .135.006.265.016.4a.373.373 0 0 0 .068.192.2.2 0 0 0 .156.062.2.2 0 0 0 .192-.109c.036-.072.052-.192.057-.363l.534.036a.56.56 0 0 1 .006.1c0 .253-.068.445-.208.57-.14.125-.337.187-.592.187-.306 0-.519-.099-.643-.29s-.187-.489-.187-.894v-.482c0-.416.062-.717.192-.909.13-.192.348-.285.654-.285.213 0 .379.041.493.12.12.088.207.217.244.363.052.223.078.456.067.685v.472l-1.048.005zm.079-1.162a.367.367 0 0 0-.063.192 5.48 5.48 0 0 0-.015.405v.197h.457v-.197c0-.135-.006-.27-.021-.405a.373.373 0 0 0-.068-.192.174.174 0 0 0-.145-.057.169.169 0 0 0-.145.057zm-11.175 1.11l-.706-2.559h.618l.249 1.158c.062.285.109.53.14.732h.02c.022-.146.068-.384.14-.727l.26-1.163h.618l-.716 2.56v1.23H7.64v-1.23h-.005zm4.905-1.536v2.767h-.488L12 13.557h-.016c-.13.255-.332.384-.596.384-.182 0-.317-.062-.405-.181-.089-.12-.13-.307-.13-.566v-2.066h.623v2.03a.659.659 0 0 0 .041.264.137.137 0 0 0 .135.078.28.28 0 0 0 .156-.052.281.281 0 0 0 .109-.124v-2.196h.623zm3.192 0v2.767h-.488l-.052-.338h-.016c-.13.255-.332.384-.596.384-.182 0-.317-.062-.405-.181-.089-.12-.13-.307-.13-.566v-2.066h.623v2.03a.659.659 0 0 0 .041.264.137.137 0 0 0 .135.078.28.28 0 0 0 .156-.052.281.281 0 0 0 .109-.124v-2.196h.623zm-1.5-.52h-.618v3.287h-.607v-3.286h-.618v-.504h1.843v.504zm3.56.961c-.036-.171-.098-.3-.181-.379a.508.508 0 0 0-.348-.12.56.56 0 0 0-.322.1.635.635 0 0 0-.228.254h-.005V9.965h-.597v3.93h.514l.062-.26h.015a.531.531 0 0 0 .218.223.69.69 0 0 0 .322.083.502.502 0 0 0 .467-.29c.099-.192.15-.499.15-.909v-.436a4.861 4.861 0 0 0-.067-.737zm-.57 1.127c0 .155-.006.316-.026.472a.508.508 0 0 1-.084.244.195.195 0 0 1-.155.072.257.257 0 0 1-.14-.036.3.3 0 0 1-.104-.109v-1.567a.425.425 0 0 1 .109-.187.224.224 0 0 1 .166-.073c.057 0 .114.026.145.073.042.078.068.16.073.249.015.166.02.332.02.498l-.005.364z" /></ svg > } }