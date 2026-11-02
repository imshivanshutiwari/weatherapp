import{utils as e}from"@windyCore";const t=e,n=(e,t)=>`<?xml version="1.0" encoding="utf-8"?>
        <gpx creator="Windy.com" version="1.1" xmlns="http://www.topografix.com/GPX/1/1" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd">
            <rte>
                <name>Windy.com ${e}</name>
                ${t.map(({point:e},t)=>`<rtept lat="${e.lat}" lon="${e.lng}">
                                <name>${t+1}</name>
                            </rtept>`).join(``)}
            </rte>
        </gpx>`,r=(e,t)=>`<?xml version="1.0" encoding="utf-8"?>
        <kml xmlns="http://www.opengis.net/kml/2.2">
        <Document>
            <name>Windy.com ${e}</name>
            <open>1</open>
            <description>
            <![CDATA[
                popisek
            ]]>
            </description>
            <Style id="path"><LineStyle><color>b20000ff</color><width>6</width></LineStyle></Style>
            <Style id="routepoint"><IconStyle><color>b20000ff</color><Icon><href>https://maps.google.com/mapfiles/kml/shapes/placemark_square.png</href></Icon></IconStyle></Style>
            <Style id="waypoint"><IconStyle><color>b20000ff</color><Icon><href>https://maps.google.com/mapfiles/kml/shapes/placemark_circle.png</href></Icon></IconStyle></Style>
            <Folder>
                <name>1. část</name>
                <visibility>1</visibility>
                <open>1</open>
                <Placemark><name>Windy.com ${e}</name><styleUrl>#path</styleUrl>
                <LineString>
                    <altitudeMode>clampToGround</altitudeMode>
                    <tessellate>1</tessellate>
                    <coordinates>
                        ${t.map(({point:e})=>`${e.lng},${e.lat},0`).join(`
`)}
                    </coordinates>
                </LineString>
                </Placemark>
            </Folder>
        </Document>
        </kml>`,i=(e,i)=>{let a=`track-${new Date().toUTCString().replace(/[:,]/g,``).replace(/ /g,`-`)}.${e}`,o=e===`gpx`?n(a,i):r(a,i);t.download(o,`application/gpx+xml`,a)};export{i as t};