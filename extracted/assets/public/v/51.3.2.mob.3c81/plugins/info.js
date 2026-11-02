import{broadcast as e,format as t,overlays as n,products as r,rootScope as i,subscription as a,trans as o,utils as s}from"@windyCore";import{t as c}from"./_shared-chunk.js";import{Bt as l,Et as u,J as d,Jt as f,K as p,Mt as m,P as h,Q as g,U as _,Ut as v,Vt as y,Wt as b,at as x,b as ee,ct as te,dt as S,et as C,f as w,ft as ne,g as T,gt as E,h as D,ht as O,n as k,nt as A,pt as j,qt as M,r as N,t as P,tt as re,w as F,wt as I,yt as L,z as R,zt as z}from"./_shared-client.js";import{t as B}from"./_shared-object-spread2.js";import"./_shared-disclose-version.js";import"./_shared-legacy.js";import{t as V}from"./_shared-svelte-store-adapter.js";import{t as H}from"./_shared-rh-pane-plugin.js";const U=i,W=s,{tsHour:G}=s,{t:K}=o,q=t,ie=e,J=r,Y=a,ae=n;var oe=c({models:()=>fe,modelsDesc:()=>pe,overlaysDesc:()=>me,poisDesc:()=>he});const X=`Very accurate model provided by the European Centre for Medium-Range Weather Forecasts.
Clear winner compared to other forecast models. Since the model is commercial, only a few companies in the World offer it.`,Z=`The HRRR is a NOAA real-time 3-km resolution, hourly updated, cloud-resolving, convection-allowing atmospheric model, initialized by 3km grids with 3km radar assimilation. Radar data is assimilated in the HRRR every 15 min over a 1-h period adding further detail to that provided by the hourly data assimilation from the 13km radar-enhanced Rapid Refresh.`,se={name:`ECMWF 9km`,provider:`ecmwf`,url:`https://www.ecmwf.int/`},Q={name:`ACCESS-C 1.5km`,provider:`bom`,url:`http://www.bom.gov.au/nwp/doc/access/NWPData.shtml`},$=`Australian Community Climate and Earth-System Simulator (ACCESS). Based upon the UK Met Office Unified Model system.
                    ACCESS-C is the numerical forecast model operated by the BoM. It is run four times daily for 7 domains
                    (Sydney, Victoria and Tasmania, Brisbane, Perth, Adelaide, Northern Queensland and Darwin) and provides forecast data
                    out to 36 hours with a horizontal resolution of approximately 1.5km and 80 vertical levels.`,ce=`Available water deficit is the amount of water in mm/inch per the visualized soil column that is missing (or is in surplus) compared to the common levels (average of 1961 to 2010) of available water at the given site and time of the year. Negative values represent less water than usual while positive values represent a higher available water amount than is common.<br /><br />Soil profile depth
    <ul>
        <li>0-40: Includes mostly topsoil. This layer usually represents most of the root zone of crops. Water deficit in this layer is critical especially for the phase of emergence and early development phases of vegetation. Drought occurrence in this layer may induce short-term drought events as well as may be part of long-term soil moisture deficit.</li>
        <li>0-100: Extends into deeper parts of the root zone. Drought occurrence in this layer induces mainly long-term soil water deficit, which has negative impacts on all vegetation development phases as well as crop yield.</li>
    </ul>`,le=`Drought intensity compares the actual amount of water available to plants with the values recorded for the given area during the same time period of the year between 1961 and 2010. Each drought intensity class represents a particular drought period return probability.
    <br /><br />No drought risk:
    <ul>
        <li>Soil: Soil is moist to touch, malleable</li>
        <li>Precipitation: Normal or higher precipitation, vegetation season course is normal</li>
        <li>Vegetation: No observable drought impacts on vegetation</li>
        <li>Water bodies: No observable drought impact on water bodies</li>
    </ul>
    <br />D0 (Minor drought):
    <ul>
        <li>Soil: Short-term reduction in soil moisture may occur</li>
        <li>Precipitation: Short local period without precipitation</li>
        <li>Vegetation: No observable drought impacts on vegetation</li>
        <li>Water bodies: No observable drought impact on water bodies</li>
    </ul>
    <br />D1 (Mild drought):
    <ul>
        <li>Soil: Soil is dry to touch, partly moldable</li>
        <li>Precipitation: Local, rather short-term precipitation deficit</li>
        <li>Vegetation: Observable short-term drought impacts on grassland, possible impacts on plant development</li>
        <li>Water bodies: Lower than usual flow on some streams and rivers</li>
    </ul>
    <br />D2 (Moderate drought):
    <ul>
        <li>Soil: Soil is dry to touch, very hard to shape</li>
        <li>Precipitation: Rather short-term precipitation deficit</li>
        <li>Vegetation: Observable drought impacts on vegetation and grassland</li>
        <li>Water bodies: Lower than usual flow on some streams and rivers</li>
    </ul>
    <br />D3 (Severe drought):
    <ul>
        <li>Soil: Soil is dry to touch, does not leave moisture on hands, not possible to shape it</li>
        <li>Precipitation: Unusually low precipitation, higher wildfire occurrence risk</li>
        <li>Vegetation: Obvious drought impacts on vegetation, expected yield loss</li>
        <li>Water bodies: Lower than usual flow on some streams and rivers, lower water levels at some water reservoirs</li>
    </ul>
    <br />D4 (Exceptional drought):
    <ul>
        <li>Soil: Soil is dry to touch, does not leave moisture on hands, not possible to shape it, long term soil moisture deficit</li>
        <li>Precipitation: Severe precipitation deficit, the course of weather increases the risk of wildfire occurrence</li>
        <li>Vegetation: Expected yield loss 20-40 %, drought impact on grassland is manifesting in cattle feed availability</li>
        <li>Water bodies: River flows and water reservoirs levels are low, small bodies of water may dry out</li>
    </ul>
    <br />D5 (Extreme drought):
    <ul>
        <li>Soil: Soil is dry and dusty, long term soil moisture deficit</li>
        <li>Precipitation: Severe long term precipitation deficit, severe risk of wildfire occurrence</li>
        <li>Vegetation: Extreme drought impacts on crops, expected yield loss of 40 % or higher, drought impact on grassland is causing cattle feed deficiency</li>
        <li>Water bodies: River flows and water bodies level on multiple-year minimum, small bodies of water may dry out</li>
    </ul>
<br />Soil profile depth
    <ul>
        <li>0-40: Includes mostly topsoil. This layer usually represents most of the root zone of crops. Water deficit in this layer is critical especially for the phase of emergence and early development phases of vegetation. Drought occurrence in this layer may induce short-term drought events as well as may be part of long-term soil moisture deficit.</li>
        <li>0-100: Extends into deeper parts of the root zone. Drought occurrence in this layer induces mainly long-term soil water deficit, which has negative impacts on all vegetation development phases as well as crop yield.</li>
    </ul>`,ue=`Readily available soil water represents the amount of water available to plants as a proportion of the maximum soil water content that can be permanently held within the given soil column. Units are % where 0% represents no water available to plants (i.e. wilting point) and 100%  represents water content held in the soil after excess water has drained away (i.e. field capacity). In general most plant species cannot survive values close to 0%, at values below 30% visible signs of water stress will be apparent on most plant species and at values below 50% (i.e. stress point) plants start to be limited by soil water availability.<br /><br />Soil profile depth
    <ul>
        <li>0-40: Includes mostly topsoil. This layer usually represents most of the root zone of crops. Water deficit in this layer is critical especially for the phase of emergence and early development phases of vegetation. Drought occurrence in this layer may induce short-term drought events as well as may be part of long-term soil moisture deficit.</li>
        <li>0-100: Extends into deeper parts of the root zone. Drought occurrence in this layer induces mainly long-term soil water deficit, which has negative impacts on all vegetation development phases as well as crop yield.</li>
    </ul>`,de=`<p>This layer indicates how conducive the water content in the biomass is to fire ignition.
        We introduce 3 different layers depending on the size of the fuel. The layers then represent the % moisture
        content of so-called 10-hour, 100-hour and 1000-hour dead fuels and the “time” represents how long it takes for
        the moisture to be significantly changed by weather. The method of calculation is based on the NFDRS basic
        equations using temperature, relative air humidity, cloud cover, snow cover, and precipitation.</p>
    <ul>
        <li>The 10-hour dead fuel moisture corresponds to the moisture of standard fuel defined as smaller diameter
            dead branches between 0.25” and 1” in diameter (ca 0.6-2.5cm) reacting to weather conditions in less than
            half a day, i.e., changing rapidly. It is really important for areas where the wildfire is carried by small
            diameter fuels.</li>
        <li>The 100-hour fuel (about 4 days) represents fuels of 1” to 2” (2.5-7.5 cm in diameter) or wrist-like
            diameter.</li>
        <li>The 1000-hour (40 days) fuel represents fuels of 3” to 8” (7.5-20+ cm), equaling logs and fallen trunks.</li>
    </ul>
    <p>Including three different diameters allows to evaluate real conditions in different environments when e.g. small
        diameter fuel may be extremely dry (i.e. ignition would be easy) while large size fuel is still wet i.e.
        hampering potential spread in such fuel. However, interpretation will always depend on the vegetation status and
        composition as well as other parameters of fuel. The provided fuel moisture values are calculated hourly for the
        first 3 days and then in 3-hourly steps.</p>
    <p>Calculated values typically range from 1% to 35% with ignitions generally occurring when values drop below 15%.
        As fuel moisture decreases below 15%, the fire danger increases. Values below 6% are of great concern especially
        when these low values are reached in all three classes with possible extreme fire behavior.</p>`,fe={ecmwf:se,efi:se,gfs:{name:`GFS 22km`,provider:`noaa`,url:`https://www.noaa.gov/`},nems:{name:`NEMS 4-12km`,provider:`meteoblue`,url:`https://www.meteoblue.com/?utm_source=windy.com&utm_medium=referral&utm_campaign=forecast-model-NEMS-layer`},iconEu:{name:`ICON-EU 7km`,provider:`dwd`,url:`https://www.dwd.de/`},iconD2:{name:`ICON-D2 2.2km`,provider:`dwd`,url:`https://www.dwd.de/`},canHrdps:{name:`HRDPS 2.5km`,provider:`msc`,url:`https://eccc-msc.github.io/open-data/readme_en/`},canRdwpsWaves:{name:`RDWPS 2.5km`,provider:`msc`,url:`https://eccc-msc.github.io/open-data/readme_en/`},czeAladin:{name:`ALADIN 2.3km`,provider:`chmi`,url:`https://www.chmi.cz/?l=en`},cams:{name:`ECMWF IFS`,provider:`copernicus`,url:`https://www.copernicus.eu/`},camsEu:{name:`CAMS regional multi-model ensemble`,provider:`copernicus`,url:`https://www.copernicus.eu/`},cmems:{name:`Copernicus Marine Environment Monitoring Service`,provider:`copernicus-marine-service`,url:`https://marine.copernicus.eu/`},hrrrAlaska:{name:`HRRR Alaska 3km`,provider:`noaa`,url:`https://www.noaa.gov/`},hrrrConus:{name:`HRRR CONUS 3km`,provider:`noaa`,url:`https://www.noaa.gov/`},bomAccess:{name:`ACCESS-G4 12km`,provider:`bom`,url:`http://www.bom.gov.au/nwp/doc/access/NWPData.shtml`},bomAccessAd:Q,bomAccessBn:Q,bomAccessDn:Q,bomAccessNq:Q,bomAccessPh:Q,bomAccessSy:Q,bomAccessVt:Q,ukv:{name:`UKV 2km`,provider:`metoffice`,url:`https://www.metoffice.gov.uk`},jmaMsm:{name:`MSM 5km`,provider:`jma`,url:`https://www.jma.go.jp/jma/indexe.html`},jmaCwmWaves:{name:`CWM 5km`,provider:`jma`,url:`https://www.jma.go.jp/jma/indexe.html`}},pe={ecmwf:X,gfs:`Basic free model provided by National Oceanic and Atmospheric Administration (NOAA) with not so good resolution.
                Compared to other models GFS can fail in mountain areas, and by forecasting clouds and precipitation.
                Since the model is free, majority of weather applications use GFS.`,nems:`Very accurate model developed by NOAA, but improved and computed by Swiss Meteoblue. Excels
                in all parameters especially in Alpine areas.`,iconEu:`High resolution model developed and operated by German DWD. One of the most modern forecast models delivering very good results in Europe.`,capAlerts:`Weather warnings issued by national weather agencies in a form of "CAP Alerts".`,canHrdps:`The High Resolution Deterministic Prediction System (HRDPS) carries
                out physics calculations to arrive at deterministic predictions of
                atmospheric elements from the current day out to 48 hours into the
                future. Data is available at a horizontal resolution of about 2.5 km.
                Predictions are performed up to four times a day.`,canRdwpsWaves:`The Regional Deterministic Wave Prediction System (RDWPS) produces wave
                forecasts using the third generation spectral wave forecast model
                WaveWatch III® (WW3). The model is forced by the 10 meters winds
                at from the High Resolution Deterministic Prediction System (HRDPS).
                Data is available at a horizontal resolution of about 2.5 km.
                Predictions are performed up to four times a day for 48 hours into
                the future.`,czeAladin:`Short-term weather forecast calculated by the numerical prediction model ALADIN
                (Aire Limitée Adaptation dynamique Développement InterNational). Developed by Meteo-France, operated by CHMI.`,cams:`IFS is a comprehensive global numerical whether prediction system developed and operated by ECMWF. As part of the Copernicus programme of
                    the European Union, the system is run at a horizontal resolution of 40 km and includes a detailed representation of atmospheric
                    composition processes.`,camsEu:`It is an ensemble of 9 state-of-the-art regional air quality models run under strict operational conditions under the coordination of
                    Météo-France, which operates under a contract with ECMWF. The regional models operate at a horizontal resolution of 10km and are driven
                    by ECMWF IFS meteorology.`,cmems:`The Copernicus Marine Environment Monitoring Service is part of the Copernicus Programme, which is an EU Programme managed by the
                    European Commission (EC) and implemented in partnership with the Member States, the European Space Agency (ESA), the European
                    Organisation for the Exploitation of Meteorological Satellites (EUMETSAT), the European Centre for medium-range Weather Forecasts
                    (ECMWF), EU Agencies and Mercator Ocean. The Programme is aimed at developing a set of European information services based
                    on satellite Earth Observation and in-situ (non-space) data.`,efi:X,hrrrAlaska:Z,hrrrConus:Z,bomAccess:`Australian Community Climate and Earth-System Simulator (ACCESS). Based upon the UK Met Office Unified Model system.
                    ACCESS-G4 is the global numerical forecast model operated by the BoM. It is run four times daily and provides forecast data out to 240 hours
                    with a horizontal resolution of approximately 12km and 70 vertical levels.`,bomAccessAd:$,bomAccessBn:$,bomAccessDn:$,bomAccessNq:$,bomAccessPh:$,bomAccessSy:$,bomAccessVt:$,jmaMsm:`The Meso-Scale Model (MSM) for weather warnings/advisories, very short-range forecasts of precipitation and
                    aviation forecasts covering Japan and its environs, providing 78 and 39-hour forecasts every 3 hours.
                    Spatial resolution of surface forecast is approximately 5km, 10km for the rest of atmosphere.`,jmaCwmWaves:`The Coastal Wave Model (CWM) providing very short-range wave, wind wave and swell forecasts covering ocean and sea around Japan.
    Forecast is 3 days long with 3 hour step and is provided every 6 hours.`},me={wind:`Average wind speed 10 meters above the surface (or at selected pressure level). Actual wind at the ground is influenced by many factors like mountains, cities, convective clouds and thermal effects.`,gust:`Wind gusts 10 meters above the surface in the last 3 hours. Actual wind at the ground is influenced by so many factors like mountains, cities and thermal effects. ECMWF model has  different method of Wind Gusts computation, resulting in higher values, than other models.`,temp:`Temperature 2m above the surface (or at the selected pressure level). Actual temperature is influenced by many factors like mountains, convective clouds and cities.`,solarpower:`Solar Power is the amount of solar radiation (also known as shortwave radiation) that reaches a horizontal plane at the surface of the Earth.<br />This parameter comprises both direct and diffuse solar radiation.<br />To a reasonably good approximation, this parameter is the model equivalent of what would be measured by a pyranometer (an instrument used for measuring solar radiation) at the surface. However, care should be taken when comparing model parameters with observations, because observations are often local to a particular point in space and time, rather than representing averages over a model grid box.`,wavePower:`Wave energy flux magnitude. Amount of energy from ocean/sea waves which are generated by local winds and associated with swell. It is the magnitude of the energy flux per unit length of wave crest, also known as the wave power per unit length of wave crest.<br /><br />In deep water, the wave energy flux magnitude can be estimated from the mean wave period and the significant wave height.`,uvindex:`<p>
        The UV index, is a measurement of the strength of the sunburn-producing UV radiation at a particular place and time.<br />
        It is a linear scale; each increase in value corresponds to a constant decrease in time to sunburn.
        </p>
        UV index values meaning:
        <ul>
            <li>0 to 2 means low danger from the Sun's UV rays for the average person.</li>
            <li>3 to 5 means moderate risk of harm from unprotected sun exposure.</li>
            <li>6 to 7 means high risk of harm from unprotected sun exposure. Protection against skin and eye damage is needed.</li>
            <li>8 to 10 means very high risk of harm from unprotected sun exposure. Take extra precautions because unprotected skin and eyes will be damaged and can burn quickly.</li>
            <li>11 or more means extreme risk of harm from unprotected sun exposure. Take all precautions because unprotected skin and eyes can burn in minutes.</li>
        </ul>`,wetbulbtemp:`The wet-bulb temperature is the lowest temperature which may be achieved by evaporative cooling of a water-wetted, ventilated surface.<br />A sustained wet-bulb temperature exceeding 35°C (95°F) is likely to be fatal even to fit and healthy people; at this temperature human bodies switch from shedding heat to the environment, to gaining heat from it.`,clouds:`Clouds and rain/snow accumulation in the last 3 hours. Please note, that rain from convective clouds cannot be included in this value, therefore observe CAPE index.`,hclouds:`High clouds is cloud coverage at an altitude between circa 6500m and the cloud tops. Cloud cover is given in percent.`,mclouds:`Medium clouds is cloud coverage at an altitude between circa 2000m and circa 6500m. Cloud cover is given in percent.`,lclouds:`Low clouds is cloud coverage at an altitude between the surface and circa 2000m.  Cloud cover is given in percent.`,fog:`Fog is a visible aerosol consisting of minute water droplets or ice crystals suspended in the air at or near the Earths surface. Fog is heavily influenced by nearby bodies of water, topography, and wind conditions.`,rh:`Relative humidity 2m above the surface (or at selected pressure level).`,cape:`Convective available potential energy, represents potential energy of atmosphere and can be indicator of forming convective clouds and storms. Values from 1000 to 2000 can indicate forming of moderate thunderstorms, and over 2000 severe ones.`,cbase:`Cloud base height is the altitude of the lowest cloud base above the ground, especially important for pilots. Model of terrain is simplified, so be cautious to use this values in mountains. On large areas, forecast model is NOT ABLE TO CALCULATE CLOUD BASE.`,rainAccu:`Total rain accumulation in the next hours or days.`,snowAccu:`Total snow accumulation in the next hours or days. To estimate snow depth, we use ratio: 1 mm of rain = 1 cm of snow.`,snowcover:`Forecasted depth and density of snow in next days. Actual snow depth will be influenced by many factors like orography, cities and so on.`,waves:`Mean wave height and its period of all wave types combined (swell and wind waves). Close to the shoreline, the actual height is influenced by the shape of the sea bottom.`,currents:`By transporting heat and energy, ocean currents play a major role in shaping the climate of Earth’s many regions. Surface currents (restricted to the upper 400 m of the ocean) are generally wind-driven and develop their typical clockwise spirals in the northern hemisphere and counter-clockwise rotation in the southern hemisphere (for warm currents). Deep ocean circulation is the result of a number of factors including temperature and salinity variations in water masses, shorelines, subsurface topography, tides, etc. Warm surface currents invariably flow from the tropics to the higher latitudes, driven mainly by atmospheric winds, as well as the earth’s rotation. Subtropical western boundary currents are warm, fast surface currents that transport a lot of water and heat of tropical origin to subpolar regions. This process is extremely important for maintaining the earth’s heat balance. Currents known as upwelling also bring cold, nutrient-rich water from the depths up to the surface. Earth’s rotation and strong seasonal winds push surface water away from some western coasts, so water rises on the western edges of continents to replace it. Marine life thrives in these nutrient-rich waters.`,currentsTide:`When a tidal current moves toward the land and away from the sea, it "floods". When it moves toward the sea away from the land, it "ebbs". These tidal currents that ebb and flood in opposite directions are called "rectilinear" or "reversing" currents. Tidal currents are the only type of current affected by the interactions of the Earth, sun, and moon. The moon’s force is much greater than that of the sun because it is 389 times closer to the Earth than the sun is. Tidal currents, just like tides, are affected by the different phases of the moon. When the moon is at full or new phases, tidal current velocities are strong and are called "spring currents". When the moon is at first or third quarter phases, tidal current velocities are weak and are called "neap currents".`,sst:`Sea surface temperature is the water temperature close to the oceans surface. Close to the shoreline, the actual value is influenced by the shape of the shoreline and the sea bottom, and also by wind and waves.`,wwaves:`Wind waves, or wind-generated waves, are surface waves that occur on the free surface of bodies of water (like oceans, seas, lakes, etc.). They result from the wind blowing over an area of water surface. Wind waves are generated by the immediate local wind – which gives them the direction. After the wind ceases to blow, wind waves are called swells.`,swell1:`A swell consists of wind-generated waves that are not significantly affected by the local wind at that time. They have been generated elsewhere or some time ago.`,swell2:`Secondary swell is produced also by winds far away but in other places than Swell1. It has different heights, directions and periods than Swell1.`,swell3:`Another secondary swell, with different heights, directions and periods than Swell1 and Swell2.`,pressure:`The mean sea-level pressure (MSLP) is the average atmospheric pressure at sea level. Atmospheric pressure, sometimes also called barometric pressure, is the pressure within the atmosphere of Earth (or that of another planet). In most circumstances atmospheric pressure is closely approximated by the hydrostatic pressure caused by the weight of air above the measurement point.`,rain:`Rain/snow accumulation in the last 3 hours.`,dewpoint:`Temperature at 2m above surface (or at selected level) at which a water vapor in air will form dew. It is a measure of atmospheric moisture.`,cloudtop:`Cloud tops height is the altitude of the cloud peak.`,deg0:`The freezing level, or 0 °C (zero-degree) isotherm, represents the altitude in which the temperature is at 0 °C (the freezing point of water).`,radar:`Reflectivity measured with doppler radar, indicating actual rain/snow/hail or amount of preticable water in clouds. Weather radar, also called Doppler weather radar, is a type of radar used to locate precipitation, calculate its motion, and estimate its type (rain, snow, hail etc.) or amount of predicable water in clouds Return echoes from targets ("reflectivity") are analyzed for their intensities to establish the precipitation rate in the scanned volume.`,thunder:`Defined as number of lighting flashes in the area of one square kilometer, in one day.`,ptype:`Precipitation types can include the character or phase of the precipitation which is falling to ground level. Precipitation can fall in either liquid or solid phases, or transition between them. Rain, Freezing rain, Mixed ice, Snow, Wet snow, Rain with snow, Ice pellets.`,visibility:`Visibility is a measure of the distance at which an object or light can be clearly discerned. Meteorological visibility refersto transparency of air: in dark, meteorological visibility is still the same as in daylight for the same air.`,cosc:`Carbon monoxide is a colorless, odorless, and tasteless gas that is slightly less dense than air. The level of CO concentrationin the troposphere is measured by a system called Parts Per Billion by Volume (PPBV).`,dustsm:`Dust generally consists of particles in the atmosphere that come from various sources such as soil, dust lifted by weather, volcanic eruptions, and air pollution. The level of dust concentration is given in micrograms (one-millionth of a gram) per cubic meter air or µg/m³ at the surface.`,capAlerts:`Weather warnings issued by national weather agencies in a form of "CAP Alerts".`,gtco3:`The ozone column corresponds to the vertically integrated quantity of ozone in the atmosphere over a unit surface area. This quantity quantifies the depth of the ozone layer and is expressed in Dobson Units. It naturally varies with latitudes and seasons between about 250 and 500 DU. The ozone layer shields the Earth surface from harmful UV radiation and is essential to life. In Austral winter and Spring, a hole forms over Antarctica and the ozone layer can fall below 100 Dobson Units.`,pm2p5:`PM2.5 are fine particulate matter in suspension in air, which can be inhaled and enter the bloodstream through the air-blood barrier in the lungs. PM2.5 increase morbidity and mortality when people are exposed to high levels over long periods. As science progresses, more impacts of fine particulate matter are being found, including on cognitive functions. PM2.5 levels are expressed in μg/m3 and the forecasts provided here correspond to near-surface values. Forecasts from the CAMS regional multi-model ensemble are provided over the domain (30N to 70N; 25W to 45E) and ECMWF IFS forecasts are used elsewhere. Due to the horizontal resolution of the systems used by CAMS, it is not possible to represent local effects due to the immediate vicinity of sources (high-traffic road, industrial plant...); forecasts correspond to so-called "background" values. The current guideline values of the World Health Organization for PM2.5 are 10 μg/m³ (annual mean) and 25 μg/m³ (daily mean). Different regulations apply across the world regarding warning and alert levels.`,no2:`NO₂ is a toxic gas that causes respiratory problems, linked to reduced lung function. NO₂ levels are expressed in μg/m³ and the forecasts provided here correspond to near-surface values. Forecasts from the CAMS regional multi-model ensemble are provided over the domain (30N to 70N; 25W to 45E) and ECMWF IFS forecasts are used elsewhere. Due to the horizontal resolution of the systems used by CAMS, it is not possible to represent local effects due to the immediate vicinity of sources (high-traffic road, industrial plant...); forecasts correspond to so-called "background" values. The current guideline values of the World Health Organization are 40 μg/m³ (annual mean) and 200 μg/m³ (hourly mean). Different regulations apply across the world regarding warning and alert levels.`,aod550:`Pollution, desert dust, sea salts, volcanic ash, smoke from wildfires are all examples of\xA0aerosols. Depending on their chemical composition and physical properties, these particles in the atmosphere can block sunlight by absorbing or scattering light. The Aerosol Optical Depth (AOD) is the degree to which aerosols prevent the transmission of light in the atmosphere. AOD is defined as the vertically integrated logarithmic extinction coefficient over a unit surface area. It is dimensionless and equals 0 in the absence of aerosol; it can exceed 1 or more in case of very heavy aerosol loads. AOD depends on wavelength. It is computed here at 550nm (yellow part of the visible spectrum).`,tcso2:`Sulphur dioxide is emitted in the atmosphere by several sources. At the global scale, a major source of sulphur dioxide comes from volcanic eruptions. Combustion of fossil fuels by human activities is also a very large source, although it has been gradually decreasing in most parts of the world thanks to increasing environmental regulations protecting human health and ecosystems. Wildfires are among the other sources. The variable presented here is the vertically integrated amount of sulphur dioxide from the ground and up to the top of the atmosphere. It is expressed in kg/m2. Forecast are provided from ECMWF IFS. Satellite observations of total column sulphur dioxide are actively assimilated in the CAMS system and influence subsequent forecasts.`,go3:`Ozone is formed in the lower atmosphere from precursor gases (nitrogen oxides and Volatile Organic Compounds) through photochemical reactions. Due to the complex set of reactions involved, ozone is generally highest in spring and summer, downwind of areas with high emissions. It is a toxic gas that causes cardiopulmonary and respiratory problems. Ozone levels are expressed in microgram/m3 and the forecasts provided here correspond to near-surface values. Forecasts from the CAMS regional multi-model ensemble are provided over the domain (30N to 70N; 25W to 45E) and ECMWF IFS forecasts are used elsewhere. Due to the horizontal resolution of the systems used by CAMS, it is not possible to represent local effects; forecasts correspond to so-called “background” values. The current guideline values of the World Health Organization is 100 microgram/m3 (maximum daily 8-hour mean). In the EU, the hourly information threshold is 180 microgram/m3 and the hourly alert threshold is 240 microgram/m3. Different regulations apply in other parts of the world regarding warning and alert levels.`,efiWind:`Statistically unusually high or low wind for given day (in UTC). ECMWF's Extreme Forecast Index based on 20 year history displays probability, that in given day there will be extreme event.`,efiTemp:`Statistically unusually cold or warm ground temperature for given day (in UTC). ECMWF's Extreme Forecast Index based on 20 year history displays probability, that in given day there will be extreme event.`,efiRain:`Statistically very dry of very rainy day (in UTC). ECMWF's Extreme Forecast Index based on 20 year history displays probability, that in given day there will be extreme event.`,ccl:`
            <p>Thermals is a layer for gliding and paragliding. It forecasts occurrence and altitude of thermals as well as occurrence of cumuli.
            Calculation is performed by Windy.com and is based on data from 3rd party models. Algorithm is experimental and is still being developed.
            We appreciate any feedback. The map colours display maximum climb (AMSL).</p>
            Symbols:
            <ul>
                <li>Small cloud - sparse cumuli</li>
                <li>Big cloud - dense cumuli</li>
                <li>Stripes - overcast</li>
                <li>Lightning - storm prediction</li>
            </ul>
        `,satellite:`Composite of various satellite images`,gustAccu:`Wind gust accumulation.`,moistureAnom40:ce,moistureAnom100:ce,drought40:le,drought100:le,soilMoisture40:ue,soilMoisture100:ue,fwi:`<p>This layer represents the potential for a wildfire (i.e., wildland fire, forest fire, vegetation fire)
        initiation and spread based on a combination of various meteorological parameters.</p>
        <p>It is based on the Fire Weather Index (FWI) system. The value indicates to which degree the weather and
        fuel conditions are conducive to the fire spread and the rating system recognizes six levels of fire spread
        danger: Very low, Low, Moderate, High, Very high, Extreme.</p>
    </ul>
    <p>As the mean value and amplitude of FWI differ depending on the local vegetation and climate context the
        specific threshold values were derived for 18 global environmental zones. Each environmental zone has been
        assigned unique distribution of FWI values and associated fire danger categories based on an extensive review
        of scientific literature.</p>
    <p>Fire spread is indication of fire weather conditions is determined based on the values of the Canadian Forest
        Fire Weather Index (FWI) calculated from temperature, relative humidity, wind, and precipitation for today and
        the next 9 days based on the ECMWF IFS prediction model.</p>
    `,dfm10h:de,dfm100h:de,dfm1000h:de,turbulence:`
        <p>
            <span
                >Our turbulence product provides objective and reliable forecasts of Clear Air
                Turbulence (CAT) intensity in space and time. We use a combination of diagnostics,
                including the popular Ellrod turbulence diagnostic (Ellrod and Knapp, 1992), as well
                as other diagnostics that represent horizontal wind deformation, vertical wind
                shear, and static instability terms. These diagnostics are calculated using forecast
                winds and temperature derived from Numerical Weather Prediction (NWP) models.</span
            >
            <span>
                The final product is expressed as the turbulence intensity with regard to the
                aircraft weight category (see table below or ICAO Annex 3) as previously converted
                to the EDR aviation turbulence metric (Sharman and Pearson, 20017).</span
            >
        </p>
        <p></p>
        <p>
            <span
                >At this stage we do not aim to predict orographically induced turbulence or
                turbulence associated with boundary layer, convection and thunderstorm clouds. As
                turbulence is a highly intermittent and anisotropic phenomenon and its forecast is
                model dependent, our turbulence product needs to be interpreted with caution and
                with a certain level of expertise.</span
            >
        </p>
        <p></p>
        <p><span>References</span></p>
        <ul>
            <li>
                <span
                    >Ellrod, G. P., and D. I. Knapp, 1992: An Objective Clear-Air Turbulence
                    Forecasting Technique: Verification and Operational Use.</span
                >
                <span class="c18">Wea. Forecasting</span> <span>,</span> <span>7</span>
                <span>, 150-165,</span>
                <span
                    ><a
                        href="https://doi.org/10.1175/1520-0434%281992%29007%3C0150%3AAOCATF%3E2.0.CO%3B2"
                        >https://doi.org/10.1175/1520-0434%281992%29007%3C0150%3AAOCATF%3E2.0.CO%3B2
                    </a></span
                >
                <span>.</span>
            </li>
            <li>
                <span
                    >Ellrod, G. P., and J. A. Knox, 2010: Improvements to an Operational Clear-Air
                    Turbulence Diagnostic Index by Addition of a Divergence Trend Term.</span
                >
                <span>Wea. Forecasting</span> <span class="calibre">,</span> <span>25</span>
                <span class="calibre">, 789-798,</span>
                <span
                    ><a href="https://doi.org/10.1175/2009WAF2222290.1"
                        >https://doi.org/10.1175/2009WAF2222290.1</a
                    ></span
                >
                <span class="c4">.</span>
            </li>
            <li><span>ICAO Annex 3</span></li>
            <li>
                <span
                    >Sharman, R. D., & Pearson, J. M. (2017). Prediction of Energy Dissipation Rates
                    for Aviation Turbulence. Part I: Forecasting Nonconvective Turbulence.</span
                >
                <span>Journal of Applied Meteorology and Climatology</span> <span>,</span>
                <span class="c18">56</span> <span>(2), 317-337.</span>
                <span
                    ><a href="https://doi.org/10.1175/JAMC-D-16-0205.1">
                        https://doi.org/10.1175/JAMC-D-16-0205.1</a
                    ></span
                >
            </li>
        </ul>`,icing:`
        <p>
            <span
                >Our icing product offers automated, objective and reliable forecasts of the
                probability of encountering icing conditions in space and time. We utilize
                meteorological factors, including air temperature, humidity, and cloud properties
                derived from Numerical Weather Prediction (NWP) models. The resulting product
                indicates the likelihood of encountering icing, which is similar to the icing
                product provided in the World Area Forecast System (WAFS) defined by ICAO Annex 3,
                mapped to the following discrete scale representing expected icing severity
                (Belo-Pereira, M., 2015):</span
            >
        </p>
        <p>
            <span>
                Since the likelihood of icing is also influenced by non-meteorological factors such
                as aircraft type and speed, we recommend using this product in conjunction with
                other available sources of icing information, such as SIGMETs, PIREPs, and satellite
                data, to enhance situational awareness.
            </span>
        </p>
        <p></p>
        <p><span>References</span></p>
        <ul>
            <li>
                <span
                    >Morcrette, C., Brown, K., Bowyer, R., Gill, P., & Suri, D. (2019). Development
                    and evaluation of in-flight icing index forecast for aviation. Weather and
                    Forecasting, 34(3), 731–750.
                    <a href="https://doi.org/10.1175/WAF-D-18-0177.1"
                        >https://doi.org/10.1175/WAF-D-18-0177.1</a
                    ></span
                >
            </li>
            <li><span>ICAO Annex 3</span></li>
            <li>
                <span
                    >Belo-Pereira, M. (2015), Comparison of in-flight aircraft icing algorithms
                    based on ECMWF forecasts. Met. Apps, 22: 705-715.
                    <a href="https://doi.org/10.1002/met.1505"
                        >https://doi.org/10.1002/met.1505</a
                    ></span
                >
            </li>
        </ul>
    `,aqi:`<p>
    The US Air Quality Index (AQI) is a standardized system used to measure and report air quality levels in a specific area. It provides a clear and easy-to-understand way for the public to assess the potential health effects of air pollution. The AQI is calculated based on the concentrations of major air pollutants, including ground-level ozone (O3), particulate matter (PM2.5 and PM10), carbon monoxide (CO), sulfur dioxide (SO2), and nitrogen dioxide (NO2).
</p>

<p>
    The AQI scale ranges from 0 to 500, with higher values indicating worse air quality and greater health risks. The scale is divided into six categories, each represented by a specific color and associated with a level of health concern:
</p>

<ul>
    <li><strong>0-50 (Good):</strong> Air quality is considered satisfactory, and air pollution poses little or no risk.</li>
    <li><strong>51-100 (Moderate):</strong> Air quality is acceptable; however, some pollutants may pose a moderate health concern for a small number of sensitive individuals.</li>
    <li><strong>101-150 (Unhealthy for Sensitive Groups):</strong> Members of sensitive groups, such as children, the elderly, and individuals with respiratory or heart conditions, may experience health effects. The general public is unlikely to be affected.</li>
    <li><strong>151-200 (Unhealthy):</strong> Everyone may begin to experience health effects, and members of sensitive groups may experience more serious effects.</li>
    <li><strong>201-300 (Very Unhealthy):</strong> Health alert: everyone may experience more serious health effects.</li>
    <li><strong>301-500 (Hazardous):</strong> Health warnings of emergency conditions. The entire population is more likely to be affected.</li>
</ul>

<p>
    The AQI is updated regularly and is widely used by government agencies, environmental organizations, and weather services to inform the public about current air quality conditions. It helps individuals make informed decisions about outdoor activities, especially during periods of high pollution.
</p>

<p>
    For more information about the US AQI and how it is calculated, visit the <a href="https://www.airnow.gov/" target="_blank" rel="noopener noreferrer">AirNow website</a>, which is managed by the United States Environmental Protection Agency (EPA).
</p>`},he={firespots:`The data on active fires is provided by <a target="_blank" rel="noopener noreferrer nofollow"
href="https://firms.modaps.eosdis.nasa.gov/">NASA FIRMS (Fire Information for Resource Management System)</a>.
The presented value is called Fire Radiative Power (FRP) and depicts the pixel-integrated fire radiative power in MW
(megawatts) over a square kilometer. To offer a comprehensive view of fires that is larger than one pixel,
we present total area of a fire cluster. A single pixel corresponds to approximately 300 meters. On the minimap, a circle
indicates the approximate location of a single fire pixel. For more information about how the data is collected, refer to
<a target="_blank" rel="noopener noreferrer nofollow" href="https://www.earthdata.nasa.gov/faq/firms-faq">FAQ page at NASA</a>.`};var ge=A(`<div class="info-description"><h3> </h3> <!></div>`),_e=A(`<div class="info-description"><h3> </h3> </div>`),ve=A(`<h3> </h3> <div class="info-description"><!></div>`,1),ye=A(`<span><!> <!> <!></span>`);function be(e,t){if(new.target)return d(B({component:be},e));f(t,!1);let n=()=>N(re,`$overlayStore`,i),r=()=>N(F,`$poisStore`,i),[i,a]=k(),o=I(),s=I(),c=I(),p=I(),h=I(),v=P(t,`product`,12),{models:x,modelsDesc:ee,overlaysDesc:te,poisDesc:A}=oe,re=V(`overlay`),F=V(`pois`),H=I(),W=I();S(()=>O(v()),()=>{u(o,x[v().ident])}),S(()=>n(),()=>{u(s,ae[n()])}),S(()=>n(),()=>{u(c,te[n()])}),S(()=>O(v()),()=>{u(p,ee[v().ident])}),S(()=>r(),()=>{u(W,A[r()])}),S(()=>(r(),K),()=>{if(r()&&U.pois[r()]&&U.pois[r()].title){let e=U.pois[r()].title;e&&u(H,K[e])}}),S(()=>(n(),E(c)),()=>{u(h,n()in K?K[n()]:E(c))}),ne();var G={get product(){return v()},set product(e){v(e),m()},$set:D,$on:(e,n)=>w(t,e,n)};T();var q=ye(),ie=z(q),J=e=>{var t=ge(),n=z(t),r=z(n,!0);b(n),R(y(n,2),()=>E(h)),b(t),j(e=>g(r,e),[()=>(E(s),L(()=>E(s).getName()))]),C(e,t)};_(ie,e=>{E(c)&&e(J)});var Y=y(ie,2),X=e=>{var t=_e(),n=z(t),r=z(n,!0);b(n);var i=y(n);b(t),j(()=>{var e;g(r,(E(o),O(v()),L(()=>E(o)?E(o).name:v()?v().modelName:``))),g(i,` ${(e=E(p))==null?``:e}`)}),C(e,t)};_(Y,e=>{E(p)&&e(X)});var Z=y(Y,2),se=e=>{var t=ve(),n=l(t),r=z(n,!0);b(n);var i=y(n,2);R(z(i),()=>E(W)),b(i),j(()=>g(r,E(H))),C(e,t)};_(Z,e=>{E(W)&&E(H)&&e(se)}),b(q),C(e,q);var Q=M(G);return a(),Q}var xe=A(`<img class="info-img-provider svelte-1auifd8"/>`),Se=A(` <br/>`,1),Ce=A(` <a target="_top"> </a>`,1),we=A(` <span> </span>`,1),Te=A(`<!> <!> <!>`,1);const Ee={hash:`svelte-1auifd8`,code:`img.info-img-provider.svelte-1auifd8 {position:absolute;right:20px;top:90px;max-width:80px;max-height:60px;opacity:0.6;}#device-mobile img.info-img-provider.svelte-1auifd8,
#device-tablet img.info-img-provider.svelte-1auifd8 {display:none;}`};function De(e,t){if(new.target)return d(B({component:De},e));f(t,!1),h(e,Ee);let n=I(),r=I(),i=P(t,`product`,12);S(()=>O(i()),()=>{u(n,fe[i().ident])}),S(()=>(E(n),O(i())),()=>{u(r,E(n)?E(n).name:i()?i().modelName+(i().modelResolution?` `+i().modelResolution+`km`:``):null)}),ne();var a={get product(){return i()},set product(e){i(e),m()},$set:D,$on:(e,n)=>w(t,e,n)};T();var o=Te(),s=l(o),c=e=>{var t=xe();j(()=>{var e,r;F(t,`src`,`${(e=`https://www.windy.com/img`)==null?``:e}/providers/${(r=(E(n),L(()=>E(n).provider)))==null?``:r}-grey.svg`),F(t,`alt`,(E(n),L(()=>E(n).provider)))}),C(e,t)};_(s,e=>{E(n)&&e(c)});var p=y(s,2),x=e=>{var t=Se(),n=l(t);v(),j(()=>{var e,t;return g(n,`${(e=(O(K),L(()=>K.MODEL)))==null?``:e}: ${(t=E(r))==null?``:t} `)}),C(e,t)};_(p,e=>{E(r)&&e(x)});var ee=y(p,2),te=e=>{var t=Ce(),r=l(t),a=y(r),o=z(a,!0);b(a),j(()=>{var e;g(r,`${(e=(O(K),L(()=>K.PROVIDER)))==null?``:e}: `),F(a,`href`,(E(n),L(()=>E(n).url))),g(o,(O(i()),L(()=>i().provider)))}),C(e,t)},k=e=>{var t=we(),n=l(t),r=y(n),a=z(r,!0);b(r),j(()=>{var e;g(n,`${(e=(O(K),L(()=>K.PROVIDER)))==null?``:e}: `),g(a,(O(i()),L(()=>i().provider)))}),C(e,t)};return _(ee,e=>{E(n)&&i()?e(te):(O(i()),L(()=>i()&&i().provider)&&e(k,1))}),C(e,o),M(a)}const Oe=(e,t,n)=>{let r=e+G*n;return{isExpired:r<t,ts:r}},ke=(e,t)=>{let n=Date.now(),r=new Date(t.update).getTime(),i=(Y.hasAny()&&e.intervalPremium?e.intervalPremium:e.interval)/60,a={last:Oe(r,n,0),lastPremium:null,next:Oe(r,n,i+1),nextPremium:null,intervals:{current:i,premium:null}};if(!Y.hasAny()&&e.intervalPremium){var o;let t=((o=e.intervalPremium)==null?0:o)/60,i=0;for(let e=0;a.last.ts+e*(t+1)*G<n;e++)i=e;a.lastPremium=Oe(r,n,i*t),a.nextPremium=Oe(r,n,(i+1)*t+1),a.intervals.premium=t}return a};var Ae=A(`<strong> </strong> `,1),je=A(`<strong> </strong> `,1),Me=A(`<section class="size-l fg-white get-premium get-premium--menu-block"><strong><!></strong> <!> <div class="centered"><div class="premium-button"> </div></div></section>`);function Ne(e,t){if(new.target)return d(B({component:Ne},e));f(t,!1);let n=P(t,`updates`,12);var r={get updates(){return n()},set updates(e){n(e),m()},$set:D,$on:(e,n)=>w(t,e,n)};T();var i=re(),a=l(i),o=e=>{var t=Me();F(t,`data-icon`,``);var r=z(t);R(z(r),()=>(O(K),O(n()),L(()=>K.SUB_INTERVAL.replace(`{{premiumInterval}}`,String(n().intervals.premium))))),b(r);var i=y(r,2),a=e=>{var t=Ae(),r=l(t),i=z(r,!0);b(r);var a=y(r);j((e,t)=>{g(i,e),g(a,` ${t==null?``:t}`)},[()=>(O(K),O(q),O(n()),L(()=>K.SUB_NEXT_UPDATE.replace(`{{nextPremiumUpdate}}`,q.countdown(n().nextPremium.ts)))),()=>(O(K),O(q),O(n()),L(()=>K.FREE_UPDATE.replace(`{{nextFreeUpdate}}`,q.countdown(n().next.ts))))]),C(e,t)},o=e=>{var t=je(),r=l(t),i=z(r,!0);b(r);var a=y(r);j((e,t)=>{g(i,e),g(a,` ${t==null?``:t}`)},[()=>(O(K),O(q),O(n()),L(()=>K.SUB_LAST_PREMIUM.replace(`{{lastPremiumUpdate}}`,q.countdown(n().lastPremium.ts)))),()=>(O(K),O(q),O(n()),L(()=>K.LAST_FREE_UPDATE.replace(`{{lastFreeUpdate}}`,q.countdown(n().last.ts))))]),C(e,t)};_(i,e=>{O(n()),L(()=>n().next.ts!==n().nextPremium.ts)?e(a):e(o,!1)});var s=y(i,2),c=z(s),u=z(c,!0);b(c),b(s),b(t),j(()=>g(u,(O(K),L(()=>K.SUB_GO)))),te(`click`,c,()=>ie.emit(`rqstOpen`,`subscription`,{promote:`frequency`,subsSource:`infoRefTimes`})),C(e,t)};return _(a,e=>{O(n()),L(()=>n().intervals.premium&&!n().next.isExpired&&n().lastPremium&&n().nextPremium&&(n().last.ts!==n().lastPremium.ts||n().next.ts!==n().nextPremium.ts))&&e(o)}),C(e,i),M(r)}var Pe=A(` <b class="inline-block svelte-18j4wn2"> <!></b> <br/>`,1),Fe=A(`- <span class="clickable dotted size-xs"> </span>`,1),Ie=A(`<h3> <!></h3> <section class="update-list"><!> <div class="inline-block svelte-18j4wn2"> <!></div> <br/> <span class="inline-block svelte-18j4wn2"> </span> <br/></section> <!>`,1);const Le={hash:`svelte-18j4wn2`,code:`.inline-block.svelte-18j4wn2 {display:inline-block;}`};function Re(e,t){if(new.target)return d(B({component:Re},e));f(t,!1),h(e,Le);let n=I(),r=P(t,`product`,12),i=P(t,`minifest`,12),a,o=q.getHoursFunction(),s=()=>{u(n,ke(r(),i())),E(n).next.isExpired&&clearInterval(a)},c=()=>{clearInterval(a),a=setInterval(s,1e3),s()};p(()=>(c(),()=>clearInterval(a))),S(()=>(O(r()),O(i())),()=>{u(n,ke(r(),i()))}),ne();var ee={get product(){return r()},set product(e){r(e),m()},get minifest(){return i()},set minifest(e){i(e),m()},$set:D,$on:(e,n)=>w(t,e,n)};T();var k=Ie(),A=l(k),N=z(A),re=y(N),F=e=>{var t=x();j(e=>g(t,`(ref ${e==null?``:e})`),[()=>(O(i()),L(()=>i().ref.replace(/^.*\d\dT(\d\d):.*$/,`$1Z`)))]),C(e,t)};_(re,e=>{O(U),L(()=>!U.isMobileOrTablet)&&e(F)}),b(A);var R=y(A,2),V=z(R),H=e=>{var t=Pe(),r=l(t),i=y(r),a=z(i),s=y(a),c=e=>{var t=x();j(()=>g(t,(O(K),L(()=>K.NOW_AT_ANY_MOMENT)))),C(e,t)},u=e=>{var t=x();j(e=>{var n;return g(t,`${(n=(O(K),L(()=>K.IN)))==null?``:n} ${e==null?``:e}`)},[()=>(O(q),E(n),L(()=>q.countdown(E(n).next.ts,{showSeconds:!0})))]),C(e,t)};_(s,e=>{E(n),L(()=>E(n).next.isExpired)?e(c):e(u,!1)}),b(i),v(2),j(e=>{var t;g(r,`${(t=(O(K),L(()=>K.MENU_D_NEXT_UPDATE)))==null?``:t} `),g(a,`${e==null?``:e},  `)},[()=>(E(n),L(()=>o(new Date(E(n).next.ts).getHours(),new Date(E(n).next.ts).getMinutes())))]),C(e,t)};_(V,e=>{E(n),L(()=>E(n).next)&&e(H)});var G=y(V),J=y(G),Y=z(J),ae=y(Y),oe=e=>{var t=Fe(),r=y(l(t)),i=z(r,!0);b(r),j(e=>g(i,e),[()=>(O(W),O(K),E(n),L(()=>W.template(K.ACTIVATE_PREMIUM_UPDATE,{number:E(n).intervals.premium})))]),te(`click`,r,()=>ie.emit(`rqstOpen`,`subscription`,{promote:`frequency`,subsSource:`infoFrequency`})),C(e,t)};_(ae,e=>{E(n),L(()=>E(n).intervals.premium)&&e(oe)}),b(J);var X=y(J,3),Z=y(X),se=z(Z,!0);return b(Z),v(2),b(R),Ne(y(R,2),{get updates(){return E(n)}}),j(e=>{var t,r,a,o;g(N,`${(t=(O(K),L(()=>K.MENU_D_UPDATED)))==null?``:t}
    ${e==null?``:e} `),g(G,` ${(r=(O(K),L(()=>K.MENU_U_INTERVAL)))==null?``:r}: `),g(Y,`${(a=(E(n),L(()=>E(n).intervals.current+` - `+(E(n).intervals.current+1))))==null?``:a} hrs `),g(X,` ${(o=(O(K),L(()=>K.MENU_D_REFTIME)))==null?``:o}: `),g(se,(O(i()),L(()=>i().ref)))},[()=>(O(q),E(n),L(()=>q.howOld({ts:E(n).last.ts,translate:!0}).replace(/ /g,`\xA0`)))]),C(e,k),M(ee)}var ze=A(`<div class="copy svelte-1erbpl9"> </div>`);const Be={hash:`svelte-1erbpl9`,code:`.copy.svelte-1erbpl9 {font-size:9px;margin-top:2em;text-align:center;padding:0 20px;opacity:0.6;}`};function Ve(e,t){if(new.target)return d(B({component:Ve},e));f(t,!1),h(e,Be);let n=P(t,`product`,12),r=`© ${new Date().getFullYear()} ECMWF: This service is based on data and products of the European Centre for Medium-range Weather Forecasts`,i=`© Generated using Copernicus Atmosphere Monitoring Service Information [${new Date().getFullYear()}]. Neither the European Commission nor ECMWF is responsible for any use of this information.`,a={ecmwf:r,cams:i,camsEu:i,ecmwfWaves:r};var o={get product(){return n()},set product(e){n(e),m()},$set:D,$on:(e,n)=>w(t,e,n)};T();var s=re(),c=l(s),u=e=>{var t=ze(),r=z(t,!0);b(t),j(()=>g(r,(O(n()),L(()=>a[n().ident])))),C(e,t)};return _(c,e=>{O(n()),L(()=>n()&&n().ident in a)&&e(u)}),C(e,s),M(o)}var He=A(`<br/> <!>`,1),Ue=A(`<!> <!> <!> <!> <!>`,1);function We(e,t){if(new.target)return d(B({component:We},e));f(t,!1);let n=()=>N(c,`$productIdent`,r),[r,i]=k(),a=I(),o=I(null),s=I(!1),c=V(`product`),p=e=>{e!=null&&e.displayOverlayInfoFirst&&u(s,e.displayOverlayInfoFirst)};S(()=>n(),()=>{u(a,J[n()])}),S(()=>E(a),()=>{E(a).hasMinifest&&E(a).loadMinifest().then(e=>u(o,e))}),ne();var m={onopen:p,$set:D,$on:(e,n)=>w(t,e,n)};T(),H(e,{get pluginTitle(){return O(K),L(()=>K.ABOUT_DATA)},children:(e,t)=>{var n=Ue(),r=l(n),i=e=>{be(e,{get product(){return E(a)}})};_(r,e=>{E(s)&&e(i)});var c=y(r,2);De(c,{get product(){return E(a)}});var u=y(c,2),d=e=>{var t=He();Re(y(l(t),2),{get product(){return E(a)},get minifest(){return E(o)}}),C(e,t)};_(u,e=>{E(o)&&e(d)});var f=y(u,2),p=e=>{be(e,{get product(){return E(a)}})};_(f,e=>{E(s)||e(p)}),Ve(y(f,2),{get product(){return E(a)}}),C(e,n)},$$slots:{default:!0}}),ee(t,`onopen`,p);var h=M(m);return i(),h}const Ge=`#device-mobile .oninfo #map-container {  transform: scale(0.93) translateY(0);  transform: scale(0.93) translateY(var(--margin-top));  border-radius: 8px;  transform-origin: top center;}#device-mobile .oninfo #search,#device-tablet .oninfo #search,#device-mobile .oninfo #plugin-picker-mobile,#device-tablet .oninfo #plugin-picker-mobile,#device-mobile .oninfo #logo-wrapper,#device-tablet .oninfo #logo-wrapper,#device-mobile .oninfo #go-premium-mobile,#device-tablet .oninfo #go-premium-mobile {  display: none !important;}#device-desktop .oninfo .right-border {  right: 400px;}#device-desktop .oninfo #logo-wrapper {  left: 200px;}#device-desktop .oninfo #map-container {  transition-delay: 0.2s;  transform: translateX(-200px);}@media only screen and (max-width: 1100px) {  #device-desktop#device-desktop .oninfo #desktop-premium-icon,  #device-desktop#device-desktop .oninfo #search-input-icons {    display: none;  }}#device-tablet .oninfo #plugin-menu,#device-tablet .oninfo #logo-wrapper,#device-tablet .oninfo #go-premium-mobile,#device-tablet .oninfo #search,#device-tablet .oninfo #plugin-mobile-ui {  display: none !important;}#device-tablet .oninfo #map-container {  transition-delay: 0.2s;  transform: translateX(-200px);}#plugin-info {  -webkit-touch-callout: auto;  user-select: text;}#plugin-info * {  -webkit-touch-callout: auto;  user-select: auto;}#plugin-info .plugin__content {  line-height: 1.6;}#device-desktop #plugin-info .plugin__content {  padding: 25px 25px 100px 25px;}#device-tablet #plugin-info .plugin__content {  padding: 20px 20px 100px 20px;}#device-mobile #plugin-info .plugin__content {  padding: 0 15px 100px 15px;}#plugin-info .plugin__content a {  text-decoration: underline;  text-decoration-style: dotted;}`;export{Ge as __css,We as default};