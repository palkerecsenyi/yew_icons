use crate :: IconProps ; # [inline (never)] pub fn simple_icons_datto (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M3.9141 10.6406c-.4547-.3844-.8766-.5485-1.4531-.5485C1.0078 10.0922 0 11.2734 0 12.9844c0 1.7203 1.0313 2.9391 2.4938 2.9391.5484 0 1.0313-.1875 1.4156-.5625v.4312h1.1813V8.0578l-1.1766.0047v2.5781zm-.1313 3.4079c-.1641.4312-.6422.7359-1.1719.7359-.8344 0-1.3688-.689-1.3688-1.7578 0-1.1109.5391-1.8047 1.4063-1.8047.525 0 1.0313.3281 1.1578.7594.0704.2109.0938.5015.0938 1.1015 0 .4735-.0328.7688-.1172.9657zm6.5437-.1266v-2.0531c0-.6656-.0703-.9235-.3609-1.2281-.3375-.3656-.9609-.5625-1.7344-.5625-.7828 0-1.5703.1875-2.3297.5719l.4922.9468c.7828-.3375 1.1719-.4312 1.7109-.4312.7266 0 1.0219.2578 1.0313.8906v.1781c-.9468.0938-1.2656.1312-1.6406.1969-1.1953.2344-1.7578.7593-1.7578 1.65 0 1.0547.9141 1.8516 2.1188 1.8516.6422 0 1.1484-.2344 1.5093-.689.1172.3843.4313.6187.9609.689l.6-.9141c-.5437-.2203-.6-.3375-.6-1.0969zm-1.1953.1031c0 .3281-.0469.4547-.2109.5953-.2109.1875-.5718.3188-.8765.3188-.5719 0-1.0453-.375-1.0453-.8344 0-.375.2485-.5953.7969-.7266.3657-.0844.6656-.1172 1.3359-.1875v.8344zm5.2313.6328.15 1.0406c-.3844.1031-.8203.1641-1.1953.1641-1.1016 0-1.5328-.4547-1.5328-1.5938v-3.0469h-1.0172v-1.0172h1.0172V8.4844l1.1813-.4219v2.1422h1.4719v1.0172h-1.4672v2.6531c0 .6891.1406.8531.7266.8531.1967 0 .4218-.0234.6655-.0703zm4.0828 0 .15 1.0406c-.3844.1031-.8203.1641-1.1953.1641-1.1016 0-1.5328-.4547-1.5328-1.5938v-3.0469H14.85v-1.0172h1.0172V8.4844l1.1813-.4219v2.1422h1.4672v1.0172h-1.4672l.0046 2.6531c0 .6891.1406.8531.7266.8531.1969 0 .4218-.0234.6656-.0703zm2.8125-4.5937c-1.5563 0-2.6016 1.1859-2.6016 2.9625 0 1.7203 1.1016 2.9156 2.6953 2.9156 1.5609 0 2.6484-1.2047 2.6484-2.9156.0001-1.7532-1.1109-2.9625-2.7421-2.9625zm.0469 4.725c-.8672 0-1.4391-.7125-1.4391-1.7813s.5859-1.8047 1.4531-1.8047c.8438 0 1.4063.7172 1.4063 1.8047 0 1.0641-.5766 1.7813-1.4203 1.7813z" /></ svg > } }