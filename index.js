!function(e,t){"object"==typeof exports&&"object"==typeof module?module.exports=t():"function"==typeof define&&define.amd?define([],t):"object"==typeof exports?exports.runRoguelike=t():e.runRoguelike=t()}(window,(function(){return function(e){function t(t){for(var n,i,o=t[0],s=t[1],a=0,u=[];a<o.length;a++)i=o[a],Object.prototype.hasOwnProperty.call(r,i)&&r[i]&&u.push(r[i][0]),r[i]=0;for(n in s)Object.prototype.hasOwnProperty.call(s,n)&&(e[n]=s[n]);for(d&&d(t);u.length;)u.shift()()}var n={},r={0:0};var i={};var o={5:function(){return{"./index.js":{__wbindgen_object_drop_ref:function(e){return n[1].exports.__wbindgen_object_drop_ref(e)},__wbindgen_string_new:function(e,t){return n[1].exports.__wbindgen_string_new(e,t)},__widl_f_set_fill_style_CanvasRenderingContext2D:function(e,t){return n[1].exports.__widl_f_set_fill_style_CanvasRenderingContext2D(e,t)},__widl_f_fill_rect_CanvasRenderingContext2D:function(e,t,r,i,o){return n[1].exports.__widl_f_fill_rect_CanvasRenderingContext2D(e,t,r,i,o)},__widl_f_fill_text_CanvasRenderingContext2D:function(e,t,r,i,o){return n[1].exports.__widl_f_fill_text_CanvasRenderingContext2D(e,t,r,i,o)},__wbindgen_debug_string:function(e,t){return n[1].exports.__wbindgen_debug_string(e,t)},__wbindgen_throw:function(e,t){return n[1].exports.__wbindgen_throw(e,t)}}}}};function s(t){if(n[t])return n[t].exports;var r=n[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,s),r.l=!0,r.exports}s.e=function(e){var t=[],n=r[e];if(0!==n)if(n)t.push(n[2]);else{var a=new Promise((function(t,i){n=r[e]=[t,i]}));t.push(n[2]=a);var u,l=document.createElement("script");l.charset="utf-8",l.timeout=120,s.nc&&l.setAttribute("nonce",s.nc),l.src=function(e){return s.p+""+e+".index.js"}(e);var d=new Error;u=function(t){l.onerror=l.onload=null,clearTimeout(f);var n=r[e];if(0!==n){if(n){var i=t&&("load"===t.type?"missing":t.type),o=t&&t.target&&t.target.src;d.message="Loading chunk "+e+" failed.\n("+i+": "+o+")",d.name="ChunkLoadError",d.type=i,d.request=o,n[1](d)}r[e]=void 0}};var f=setTimeout((function(){u({type:"timeout",target:l})}),12e4);l.onerror=l.onload=u,document.head.appendChild(l)}return({2:[5]}[e]||[]).forEach((function(e){var n=i[e];if(n)t.push(n);else{var r,a=o[e](),u=fetch(s.p+""+{5:"16151a867fc0365a2eb8"}[e]+".module.wasm");if(a instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(u),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(u,a);else{r=u.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)}))}t.push(i[e]=r.then((function(t){return s.w[e]=(t.instance||t).exports})))}})),Promise.all(t)},s.m=e,s.c=n,s.d=function(e,t,n){s.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},s.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},s.t=function(e,t){if(1&t&&(e=s(e)),8&t)return e;if(4&t&&"object"==typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(s.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)s.d(n,r,function(t){return e[t]}.bind(null,r));return n},s.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return s.d(t,"a",t),t},s.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},s.p="",s.oe=function(e){throw console.error(e),e},s.w={};var a=window.webpackJsonprunRoguelike=window.webpackJsonprunRoguelike||[],u=a.push.bind(a);a.push=t,a=a.slice();for(var l=0;l<a.length;l++)t(a[l]);var d=u;return s(s.s=0)}([function(e,t,n){const r="roguelike_game";class i{constructor(e,t,n){this.rngSeed=n,t.style.display="flex",this.el=t.querySelector(`#${r}`),this.el&&this.el.remove(),"canvas_2d"===e?(this.el=document.createElement("canvas"),this.el.width=800,this.el.height=500,this.canvasCtx=this.el.getContext("2d"),this.canvasCtx.font="normal 12px monospace"):(this.el=document.createElement("pre"),this.el.style.padding=0,this.el.style.lineHeight=1,this.el.style.fontFamily="'Courier New', Courier, monospace"),this.el.id=r,t.appendChild(this.el),this.render=this.render.bind(this),this.mouseMove=this.mouseMove.bind(this),this.keyDown=this.keyDown.bind(this),this.run()}async run(){const{GameData:e}=await Promise.all([n.e(1),n.e(2)]).then(n.bind(null,1));this.game=e.new(this.rngSeed),this.iter=0,window.requestAnimationFrame(this.render),window.addEventListener("mousemove",this.mouseMove),window.addEventListener("keydown",this.keyDown)}render(){this.iter++%8==0&&(this.game.tick(),this.canvasCtx?this.game.render_to_canvas(this.canvasCtx):this.el.textContent=this.game.render_to_string()),window.requestAnimationFrame(this.render)}mouseMove(e){if(this.el.offsetWidth&&this.el.offsetHeight){const t=(e.x-this.el.offsetLeft)/this.el.offsetWidth,n=(e.y-this.el.offsetTop)/this.el.offsetHeight;t>=0&&n>=0&&t<=1&&n<=1&&this.game.move_mouse(t,n)}}keyDown(e){this.game.press_key(e.which)}dispose(){window.removeEventListener("mousemove",this.mouseMove),window.removeEventListener("keydown",this.keyDown),this.el.remove()}}e.exports=(e={})=>{const t=e.renderMode||"text",n=e.containerId?document.getElementById(e.containerId):document.body,r=e.seed?(e=>{let t=0;for(let n=0;n<e.length;n++)t=(t<<5)-t+e.charCodeAt(n),t|=0;return t})(e.seed.toString()):Math.floor(4294967295*Math.random());if(e.containerId&&!n)throw new Error(`Cannot find element with id containerId="${containerId}"`);return new i(t,n,r)}}])}));