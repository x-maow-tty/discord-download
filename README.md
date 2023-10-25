# Discord Attachment Downloader
A tool to automatically scan a Discord data package and download all attachments.

**Notes:**
- This will not be maintained.
- This code is not great, but it works. It has been tested on my own data package.
- You will likely be rate-limited by Discord.
- This does not preserve the original file name, you will have to do that manually or through your own tool.
  - This is technically a good thing as the original file names have a chance to conflict (i.e. clipboard-pasted images are always named `unknown.png`) 
- This tool is decently fast, it was able to go through my 1,000 entry data package in just a few minutes without even being compiled in release. Thank you, Rayon.
  - It could probably be made faster if I was willing to add async (for `reqwest`), but I just didn't have the time to figure out how that would work in this code.

## Usage
Run this tool in the `messages` folder of your Discord data package, obtainable through the `Privacy and Safety` tab in Discord's settings.