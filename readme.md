# 1. 如何硬生生解析和手写过程宏

JSON Schema 转化为Rust的struct

百分之80的宏编程都与数据转换有关

## JSon Schema

## Jinja2语言基础

注意：Jinja2模版语言，是不区分缩进的。实际上所有模版语言都不区分缩紧。

常用标记：
```
注释：`{# 这是注释 #}`
变量：`{{ post.title }}`，或字典元素`{{your_dict['key']}}`，或列表`{{your_list[0]}}`
多行代码块：`{% 开始 %} HTML标签 {% 结束 %}`
```

### Delimiters（分隔符）
```
{% … %} 语句（[Statements](http://jinja.pocoo.org/docs/dev/templates/#list-of-control-structures)）
{{ … }} 打印模板输出的表达式（[Expressions](http://jinja.pocoo.org/docs/dev/templates/#expressions)）
{# … #} 注释
# … ## 行语句（[Line Statements](http://jinja.pocoo.org/docs/dev/templates/#line-statements)）
```

### 转义
简单的使用单引号进行转义

对于较大的段落，使用raw进行转义
```
{% raw %}
     <ul>
     {% for item in seq %}
         <li>{{ item }}</li>
     {% endfor %}
     </ul>
{% endraw %}
```
包含 > 、 < 、 & 或 " 字符的变量，必须要手动转义, 使用e 顾虑器可以转义这些。
```
{{ user.username | e }} 
```

### 空白控制
在开始或结束放置一个减号（ - ），可以移除块前或块后的空白。
```
{% for item in seq -%}
    {{ item }}
{%- endfor %}

{%- if foo -%}...{% endif %}
```

### Variables（变量）
除了普通的字符串变量，Jinja2还支持列表、字典和对象，你可以这样获取变量值：
```
{{ mydict['key'] }}
{{ mylist[3] }}
{{ mylist[myintvar] }}
{{ myobj.somemethod() }}
```
获取一个变量的属性有两种方式：
```
{{ foo.bar }}
{{ foo['bar'] }}
```
这两种方法基本相同（深层次的区别可以暂不考虑）
如果变量或属性不存在，会返回一个未定义值。

### Filter 过滤器()
一个filter过滤器的本质就是一个function函数。使用格式为：变量名 | 函数。 它做到的就是，把变量传给函数，然后再把函数返回值作为这个代码块的值。

如：
```
<!-- 带参数的 -->
{{变量 | 函数名(*args)}}

<!-- 不带参数可以省略括号 -->
{{变量 | 函数名}}
```
链式调用（管道式）： 和命令行的pipline管道一样，可以一次调用多个函数（过滤器），如：
```
{{ "hello world" | reverse | upper }}
```
文本块调用（将中间的所有文字都作为变量内容传入到过滤器中）：
```
{% filter upper %}
    一大堆文字
{% endfilter %}
```
> Jinja2常用过滤器

字符串操作：
```
safe：禁用转义
<p>{{ '<em>hello</em>' | safe }}</p>

capitalize：把变量值的首字母转成大写，其余字母转小写
<p>{{ 'hello' | capitalize }}</p>

lower：把值转成小写
<p>{{ 'HELLO' | lower }}</p>

upper：把值转成大写
<p>{{ 'hello' | upper }}</p>

title：把值中的每个单词的首字母都转成大写
<p>{{ 'hello' | title }}</p>

reverse：字符串反转
<p>{{ 'olleh' | reverse }}</p>

format：格式化输出
<p>{{ '%s is %d' | format('name',17) }}</p>

striptags：渲染之前把值中所有的HTML标签都删掉
<p>{{ '<em>hello</em>' | striptags }}</p>

truncate: 字符串截断
<p>{{ 'hello every one' | truncate(9)}}</p>
```

列表操作：
```
first：取第一个元素
<p>{{ [1,2,3,4,5,6] | first }}</p>

last：取最后一个元素
<p>{{ [1,2,3,4,5,6] | last }}</p>

length：获取列表长度
<p>{{ [1,2,3,4,5,6] | length }}</p>

sum：列表求和
<p>{{ [1,2,3,4,5,6] | sum }}</p>

sort：列表排序
<p>{{ [6,2,3,1,5,4] | sort }}</p>
List of Builtin Filters
```
### Tests（测试，判断）
Jinja2提供的tests可以用来在语句里对变量或表达式进行测试，如果要测试一个变量，可以在变量后加上“is”和test名，比如：
```
{% if user.age is equalto 42 %} {# 这里也可以写成... is equalto(42) #}
Ha, you are 42!
{% endif %}
```
如果要传入参数，可以在test后增加括号，也可以直接写在后面。

常用的test（未说明的均返回True或False）：

+ boolean
+ defined
+ equalto
+ escaped
+ none
+ sequence
+ string
+ number
+ reverse
+ replace

[List of Builtin Tests](https://link.zhihu.com/?target=https%3A//jinja.palletsprojects.com/en/master/templates/%23list-of-builtin-tests)

### For/If (列表控制结构)
控制结构指的是所有控制程序流程的东西——条件语句（即 if/elif/else）、for 循环以及宏和块之类的东西。使用默认语法，控制结构出现在块内。

[List of Control Structures](https://link.zhihu.com/?target=https%3A//jinja.palletsprojects.com/en/master/templates/%23list-of-control-structures)

#### For
遍历序列中的每个项目。例如，要显示名为 users 的变量中提供的用户列表：
```
<h1>Members</h1>
<ul>
{% for user in users %}
  <li>{{ user.username|e }}</li>
{% endfor %}
</ul>
```
由于模板中的变量保留了它们的对象属性，因此可以迭代像 dict 这样的容器：
```
<dl>
{% for key, value in my_dict.items() %}
    <dt>{{ key|e }}</dt>
    <dd>{{ value|e }}</dd>
{% endfor %}
</dl>
```
### 条件过滤

```
{% for dir in data_dirs if dir != "/" %}
data_dir = {{ dir }}
{% else %}
# no data dirs found
{% endfor %}
```

#### 循环索引

loop.index: 循环当前迭代(从1开始)。
loop.index0: 循环当前迭代(从0开始)。
loop.revindex: 循环迭代的数量(从1开始)。
loop.revindex0: 循环迭代的数量(从0开始)。
loop.first: 是否为迭代的第一步。
loop.last: 是否为迭代的最后一步。
loop.length: 序列中元素的数量。
#### If
Jinja 中的 if 语句类似于 Python 的 if 语句。在最简单的形式中，您可以使用它来测试变量是否已定义，不为空且不为假：
{% if users %}
<ul>
{% for user in users %}
    <li>{{ user.username|e }}</li>
{% endfor %}
</ul>
{% endif %}
```
对于多个分支，可以像在 Python 中一样使用 elif 和 else。您也可以在那里使用更复杂的表达式：
```
{% if kenny.sick %}
    Kenny is sick.
{% elif kenny.dead %}
    You killed Kenny!  You bastard!!!
{% else %}
    Kenny looks okay --- so far
{% endif %}
```
### Ansible Jinja2 模版使用
更多用法可以阅读参考文章中的链接
variables: 可以输出数据

{{ my_variable }}
{{ some_dudes_name | capitalize }}
statements: 可以用来创建条件和循环等等
```
{% if my_conditional %}
    xxx
{% endif %}

{% for item in all_items %}
    {{ item }}
{% endfor %}
```
从上文中第二个variable的例子中可以看出，Jinja2支持使用带过滤器的Unix型管道操作符。有很多的内置过滤器可供使用。

我们可以仅仅用一堆简单if和for就可以建立建立几乎任何的常规配置文件。不过如果你有意更进一步，Jinja2 Documentation包含了很多有趣的东西可供了解。我们可以看到Ansibe允许在模板中使用一些额外的模版变量。

按照Ansible template_module, 我们模板的示例：
```
- name: Create Nginx SSL configuration
  template:
    src: "nginx_ssl.j2"
    dest: "/etc/nginx/sites-available/{{ project_name }}"
```
我们同样可以发现在Ansible Facts中有很多可用的Ansible变量。

参考：
https://ansible.leops.cn/basic/Jinja2/
[知乎用户@王奥](https://www.zhihu.com/people/wsgzao)

