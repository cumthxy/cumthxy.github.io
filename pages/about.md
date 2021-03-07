---
layout: page
title: About
description: 打码改变世界
keywords: Huang xingyu , 黄星宇
comments: true
menu: 关于
permalink: /about/
---

我是黄星宇

坚信熟能生巧，努力改变人生。

## 联系

<ul>
<li>
邮箱：2201627221@qq.com
</li>
</ul>


## Skill Keywords

{% for skill in site.data.skills %}
### {{ skill.name }}
<div class="btn-inline">
{% for keyword in skill.keywords %}
<button class="btn btn-outline" type="button">{{ keyword }}</button>
{% endfor %}
</div>
{% endfor %}
