# afmelder-app
Small hacked-up site to generate excuses to skip the daily (dutch language).

## What is it?
Press the button for a random and very compelling reason to not attend the daily.

## How is it working?
The frontend is a simple Hugo site with a little bit of Javascript code in it.
The Javascript does a simple api call and put the response on the site.

The back-end consist of an actix web server behind a load balancer.
It generates a random response from a vector of strings filled with compelling reasons.
Data transfer is with advanced(tm) JSON formatting technology.
The back-end can be used standalone to integrate in other applications, it's not tied to the frontend.

## How to make it work?
This is a hacked-up mess and there isn't a nice installer. To make this work, do:
* find/have/arrange/steal a VPS, I used one with a Debian Bullseye image on it.
* Install a load balancer (I used nginx) or use one from a Cloud something.
* Compile the back-end (basically `cargo build --release`) and copy the afmelder-app from target/ over to VPS
* Install afmelder.service in systemd of VPS and start back-end service
* "Compile" frontend with `hugo`, just like the Hugo RTFM says.
* Copy over the site found in public/ to /var/www/afmelder-app on the VPS, probably with scp
* Ready! enjoy the site.

This is in no-way a comprehensive install guide, you probably need to understand Rust (well, Cargo at least) and Hugo to fix everything I didnt tell you.

## Pics or it didn't happen
Well okay, screenshots are mandatory nowadays. 

Yes, it's just a button. And a text.

![Hou jezelf Cyberdicht](screenshot.png)

Happy now? Ok let's move along.

## Contributions and Code of Conduct
For real? You _can_ submit pull requests and stuff. If you want. 
And issues, just submit any issues you have. 
There is a big chance I use my advanced /dev/null technology to this kind of stuff.

It's just a small hack, come on. No support and no nothing of course. But feel free to interact if necessary.

## Licensing and referencing
This whole thing is MIT licensed, so do the F what you want with it but don't come sueing me if anything breaks.

The front-end is based on an altered ("butchered" or "hacked-up") version of the wonderful charlolamode theme,
which is also MIT licensed, so this whole damn thing is one big MIT party licensewise.

Charlolamode theme: https://themes.gohugo.io/themes/hugo-theme-charlolamode/
