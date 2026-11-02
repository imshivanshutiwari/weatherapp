import{broadcast as e,fetch as t,geolocation as n,map as r,picker as i,pluginsCtrl as a,rootScope as o,startup as s,store as c,utils as l}from"@windyCore";import{G as u,J as d,Jt as f,K as p,P as m,Ut as h,Vt as g,Wt as _,b as v,ct as y,et as b,f as x,g as S,h as C,nt as w,qt as T,zt as E}from"./_shared-client.js";import{t as D}from"./_shared-object-spread2.js";import"./_shared-disclose-version.js";import"./_shared-legacy.js";const{isMobile:O}=o,{wait:k}=l,A=c,j=e,{getArticle:M,getLiveAlerts:N,getPromo:P}=t,{centerMap:F}=r,{emitter:I}=i,{getHomeLocation:L}=n,{addDefaultListeners:R,back2home:z,closeAllStartupPlugins:B,showStartupElements:V,unsetShouldBeHidden:H}=s,{closeAllPlugins:U}=a,W=`<div class="desktop-promo layout--centered">
    <!-- Replace height:auto with arbitrary value (e.g. 200px) and object-position to control which part gets cropped -->
    <p class="image-container" style="height: auto">
        <img
            class="image_resized"
            style="object-fit:cover; width:100%; height:100%; object-position: top center;"
            src="https://i.windy.com/stories-media/promotions/content/html-LuLrIFMs/1775557811121-3710.webp"
            alt="Weather intelligence built for pilots"
        />
    </p>
    <div class="content-row">
        <div class="caption">Weather intelligence built for pilots</div>
        <p class="text-block">
            ⛅️ Premium forecasts, 🌬️ wind layers, and ✈️ route planning — everything you need for a
            confident pre-flight and en-route decisions
        </p>
        <div class="button button--orange">Go Premium</div>
    </div>
</div>
`,G=`<div class="mobile-promo layout--columns">
    <p class="image-container" style="max-width:150px;">
        <img
            class="image_resized"
            style="height:100%;object-fit:cover;width:100%;"
            src="https://i.windy.com/stories-media/promotions/content/html-hYC7r32r/1775548249974-5872.webp"
            alt="Weather intelligence built for pilots"
        />
    </p>
    <div class="content-column">
        <div style="text-align:left;">
            <span style="font-size:1.5em;">Weather intelligence built for pilots</span>
        </div>
        <div style="text-align:left;">
            <p>
                Premium <strong>forecasts</strong>, <strong>wind</strong> layers, and
                <strong>route planning</strong> — everything you need for a confident pre-flight and
                en-route decisions
            </p>
        </div>
        <div class="button button--orange">Go Premium</div>
    </div>
</div>
`;var K=w(`<div class="startup-element startup-debug size-xs centered fg-white svelte-52a73w"><div class="button" data-ignore="hp">1. Obsolete App</div> <div class="button" data-ignore="hp">2. Live Alerts</div> <div class="button" data-ignore="hp">3. Article</div> <div class="button" data-ignore="hp">4. Featured Promo</div> <br/> <div>Use picker to find places with possible live alerts.</div></div>`);const q={hash:`svelte-52a73w`,code:`.startup-debug.svelte-52a73w {background:rgba(0, 0, 0, 0.18);padding:7px 10px;line-height:1.4;}`};function J(e,t){if(new.target)return d(D({component:J},e));f(t,!1),m(e,q);let n=async e=>{if(e!=null&&e.id)try{let{data:t,status:n}=await P(await L(),e.id);if(B(!1),z(),n===200&&t)j.emit(`rqstOpen`,`startup-promos`,{data:t,typeOfPromo:`featured`});else{let e={id:`dummyPromo`,type:`html`,content:O?G:W,targetUrl:`https://www.windy.com/subscription`};j.emit(`rqstOpen`,`startup-promos`,{data:e,typeOfPromo:`featured`})}}catch(t){console.error(`Failed to load featured promo with id:`,e.id,t)}},r=async()=>{B(!1),await k(200);try{let{data:e}=await M(await L());e&&e.id&&e.title?(j.emit(`rqstOpen`,`startup-articles`,{data:e}),console.log(`Article loaded successfully:`,e)):(console.error(`Invalid article data received:`,e),j.emit(`rqstOpen`,`startup-articles`,{data:{id:40911,author:{username:`Jari Sochorová`,userslug:`jari-sochorová`,reputation:1114,profile:`https://community.windy.com/user/jari-sochorová`,avatar:`https://ims-s.windy.com/account/images/14777650-profileavatar.png?1747752873593`},coverPhoto:{src:`https://gallery.windy.com/albums/a/Summer-in-City.webp`,useCustom:!0},slug:`summer-in-the-city-heatwaves-in-urban-areas-their-impacts-and-future-challenges`,importance:`windyAnnounce`,subtitle:null,title:`Summer in the city: Heatwaves in urban areas and their impacts (test`,updated:`2025-07-24T12:34:10.414Z`,ranking:10}}))}catch(e){console.error(`Error loading article:`,e)}},i=[{type:`rain`,title:`🌧️ Heavy Rain`,body:`Heavy rain likely to start in the next hour`,updated:1753944049767},{type:`storm`,distanceKm:0,angleDeg:281,title:`⚡ Storm Alert`,body:`A storm is close to you`,lat:37.64903,lon:-94.96582,updated:1753944064315}],a=async e=>{var t;let{data:n}=await N(e);(n==null||(t=n.alerts)==null?void 0:t.length)>0?(console.log(`Live alerts found for`,e,n),R(),U(`startup-weather`),A.set(`timestamp`,Date.now()),H(),V(e),F(D(D({},e),{},{zoom:5}))):console.log(`No live alerts found for`,e)};p(()=>{I.on(`pickerMoved`,a)}),u(()=>{I.off(`pickerMoved`,a)});var o={onopen:n,$set:C,$on:(e,n)=>x(t,e,n)};S();var s=K(),c=E(s),l=g(c,2),w=g(l,2),Y=g(w,2);return h(4),_(s),y(`click`,c,()=>{B(!1),j.emit(`rqstOpen`,`startup-promos`,{typeOfPromo:`obsoleteApp`})}),y(`click`,l,async()=>{B(!1);let e=await L();j.emit(`rqstOpen`,`startup-live-alerts`,{alerts:i,coords:e})}),y(`click`,w,r),y(`click`,Y,()=>{B(!1);let e={id:`dummyPromo`,type:`html`,content:O?G:W,targetUrl:`https://www.windy.com/subscription`};j.emit(`rqstOpen`,`startup-promos`,{data:e,typeOfPromo:`featured`})}),b(e,s),v(t,`onopen`,n),T(o)}const Y=`#plugin-startup-debug {  transition: opacity 0.5s;  opacity: 0;  position: relative;}#plugin-startup-debug.open {  opacity: 1;}.onsearch-my-location #plugin-startup-debug,.onsearch #plugin-startup-debug {  display: none !important;}#plugin-startup-debug .closing-x {  display: block;  background: none;  font-size: 20px;  top: -5px;  right: -5px;}`;export{Y as __css,J as default};