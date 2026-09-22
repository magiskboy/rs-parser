START
-----
{               -> ADD_LBRACKET()
}               -> ADD_RBRACKET()
:               -> ADD_COLON()
,               -> ADD_COMMA()
"               -> IN_STRING("")
digit           -> IN_NUMBER("")
t               -> IN_TRUE
f               -> IN_FALSE
n               -> IN_NULL
<whitespace>    -> SKIP_WHITESPACE
_               -> STOP(e = reason)


ADD_LBRACKET
------------
{               -> ADD_LBRACKET()
}               -> ADD_RBRACKET()
:               -> ADD_COLON()
,               -> ADD_COMMA()
"               -> IN_STRING("")
digit           -> IN_DIGIT("")
<whitespace>    -> SKIP_WHITESPACE
_               -> STOP(e = reason)


ADD_RBRACKET
------------
{               -> ADD_LBRACKET()
}               -> ADD_RBRACKET()
:               -> ADD_COLON()
,               -> ADD_COMMA()
"               -> IN_STRING("")
<whitespace>    -> SKIP_WHITESPACE
_               -> STOP(e = reason)

ADD_LBRACE
------------
{               -> ADD_LBRACKET()
}               -> ADD_RBRACKET()
:               -> ADD_COLON()
,               -> ADD_COMMA()
"               -> IN_STRING("")
<whitespace>    -> SKIP_WHITESPACE
_               -> STOP(e = reason)


ADD_RBRACE
------------
{               -> ADD_LBRACKET()
}               -> ADD_RBRACKET()
:               -> ADD_COLON()
,               -> ADD_COMMA()
"               -> IN_STRING("")
<whitespace>    -> SKIP_WHITESPACE
_               -> STOP(e = reason)


IN_STRING(string)
------------
"               -> ADD_STRING(string)
_               -> IN_STRING(string = string + _)

SKIP_WHITESPACE
---------------
{               -> ADD_LBRACKET()
}               -> ADD_RBRACKET()
:               -> ADD_COLON()
,               -> ADD_COMMA()
"               -> IN_STRING("")
digit           -> IN_NUMBER("")
t               -> IN_TRUE
f               -> IN_FALSE
n               -> IN_NULL
<whitespace>    -> SKIP_WHITESPACE
_               -> STOP


IN_NUMBER(string)
------------
digit           -> IN_NUMBER(string = string + digit)
e               -> if string[-1] == e STOP(e = reason) else IN_NUMBER(string + e)
_               -> STOP(e = reason)


IN_TRUE
----------
r               -> if string == t IN_TRUE(string + r) else STOP(e = reason)
u               -> if string == tr IN_TRUE(string + u) else STOP(e = reason)
e               -> if string == tru ADD_TRUE(string + e) else STOP(e = reason)
_               -> STOP(e = reason)


IN_FALSE
-----------
a               -> if string == f IN_FALSE(string + a) else STOP(e = reason)
l               -> if string == fa IN_FALSE(string + l) else STOP(e = reason)
s               -> if string == fal IN_FALSE(string + s) else STOP(e = reason)
e               -> if string == fals IN_FALSE(string + e) else STOP(e = reason)
_               -> STOP(e = reason)


IN_NULL
----------
u               -> if string == n IN_FALSE(string + u) else STOP(e = reason)
l               -> if string == nu IN_FALSE(string + l) else STOP(e = reason)
l               -> if string == nul IN_FALSE(string + l) else STOP(e = reason)
_               -> STOP(e = reason)

