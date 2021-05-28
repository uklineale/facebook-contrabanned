[![Build Status](https://travis-ci.com/uklineale/facebook-contrabanned.svg?branch=master)](https://travis-ci.com/uklineale/facebook-contrabanned)

# Contra-banned

Contra-banned is a selective redirect server that can circumvent Facebook's post censorship algorithm. It works by having the user create a proxy set. A proxy set is a pair of URLs; one fake, one real. Facebook crawlers get sent to the fake URL, and real users get sent to the real URL. 


## Usage
Currently, the service is only available as an API.
To create your proxy set, send a POST request to https://facebook-contrabanned.herokuapp.com with a JSON body of 
```json
{
	"real_url" : "https://www.reddit.com/r/AskReddit/comments/2o2a54/what_actually_controversial_opinion_do_you_have/",
	"fake_url" : "https://mir-s3-cdn-cf.behance.net/project_modules/max_1200/6338be29271550.5605513c61a2a.jpg"
}
```

The user will get a response with the link to their proxy set. Post this link on Facebook.

[Here's what users see on Facebook,](https://imgur.com/SYTgOMd) and [here's where users ends up.](https://imgur.com/hVHvBZC)


## But why?
I probably don't agree with the things this service may be used for, but I disagree with censorship more strongly.

My only intention is to rectify the distortions social media creates and keep the peace. If you don't believe in free speech for people you disagree with, you don't believe in free speech at all. Social media sites create bubbles where people do not confront dissenting ideas, and depriving people of practice dealing with confrontation further weakens a democracy. A democracy without civil discussion can only end in civil war. 

Those that cannot disagree without being disagreeable keep no peace. They have become the consenting fomentors of a civil war they will watch from the very same phones that manufactured their consent.


## TODO
- Add a database to persist proxy sets
- Setup honeypots and collect user agents into an analytics database
- Figure out how to know untrusted user agents (is this a priori knowledge?)
- Start screening by CIDRs
- Make a UI
