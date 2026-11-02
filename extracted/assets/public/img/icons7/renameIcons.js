// Renaming filenames
var fs = require('fs')

var dirs = [
	'png_25@2x'
	,'png_25px'
	,'png_27@2x'
	,'png_27px'
	,'png_56@2x'
	,'png_56px'
	,'svg'
]

var files = [
'jasno',1
,'jasno_mlha',22
,'mlha',17
,'polojasno',3
,'polojasno_dest se snehem',12
,'polojasno_dest',6
,'polojasno_prehanky',19
,'polojasno_snih',9
,'polojasno_bourka_dest',21
,'skorojasno',2
,'skorojasno_dest se snehem',11
,'skorojasno_dest',5
,'skorojasno_prehanky',18
,'skorojasno_snih',8
,'zatazeno',4
,'zatazeno_bourka_dest se snehem',16
,'zatazeno_bourka_dest',14
,'zatazeno_bourka_snih',15
,'zatazeno_dest se snehem',13
,'zatazeno_dest',7
,'zatazeno_prehanky',20
,'zatazeno_snih',10 ]

// Removing all current symlinks
// find -L . -type l -exec rm {} \;

dirs.forEach( dir => {

	let suff = /png/.test(dir) ? 'png' : 'svg'

	process.chdir(dir)

	for (var i = 0; i < files.length; i +=2 ) {

		let fl = files[i]
		, num  = files[i+1]

		try {

			fs.symlinkSync( `${fl}.${suff}`, `${num}.${suff}`)
			fs.symlinkSync( `${fl}_vitr.${suff}`, `${num}_alert.${suff}`)


			if( fl == 'mlha' || /zatazeno/.test(fl) ) {

				fs.symlinkSync( `${fl}.${suff}`, `${num}_night.${suff}`)
				fs.symlinkSync( `${fl}_vitr.${suff}`, `${num}_night_alert.${suff}`)

			} else {

				fs.symlinkSync( `noc_${fl}.${suff}`, `${num}_night.${suff}`)
				fs.symlinkSync( `noc_${fl}_vitr.${suff}`, `${num}_night_alert.${suff}`)

			}

			console.log(`In ${dir} symlinked ${fl} to ${num}`)

		} catch(e) {

			console.error('Failed to rename', fl, 'to', num )

		}
	}

	process.chdir('..')
})
