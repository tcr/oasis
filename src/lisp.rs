#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::*;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Exprs<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Box<Expr>>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Exprs((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        _28_3cExpr_3e_29((usize, Box<Expr>, usize)),
        _28_3cExpr_3e_29_2a((usize, ::std::vec::Vec<Box<Expr>>, usize)),
        _28_3cExpr_3e_29_2b((usize, ::std::vec::Vec<Box<Expr>>, usize)),
        Expr((usize, Box<Expr>, usize)),
        Exprs((usize, Vec<Box<Expr>>, usize)),
        Num((usize, i32, usize)),
        Term((usize, String, usize)),
        ____Exprs((usize, Vec<Box<Expr>>, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym0));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym0));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym0));
            }
            None => {
                let __start: usize = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action14(input, &__start, &__end);
                let __nt = __Nonterminal::Exprs((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_29_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Exprs(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Box<Expr>>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym1));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __nt = __Nonterminal::Exprs((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Box<Expr>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __nt = __Nonterminal::_28_3cExpr_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<Box<Expr>>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____Exprs((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, i32, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __start = __sym0.as_ref().unwrap().2.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action14(input, &__start, &__end);
                let __nt = __Nonterminal::Exprs((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Exprs(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __nt = __Nonterminal::Term((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Box<Expr>>, usize)>,
        __sym1: &mut Option<(usize, Box<Expr>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_3cExpr_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Box<Expr>>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __nt = __Nonterminal::Exprs((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Box<Expr>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __nt = __Nonterminal::_28_3cExpr_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Vec<Box<Expr>>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, i32, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __start = __sym0.as_ref().unwrap().2.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action14(input, &__start, &__end);
                let __nt = __Nonterminal::Exprs((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Exprs(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __nt = __Nonterminal::Term((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Box<Expr>>, usize)>,
        __sym1: &mut Option<(usize, Box<Expr>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_3cExpr_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Vec<Box<Expr>>, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Vec<Box<Expr>>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Vec<Box<Expr>>, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Exprs::parse_Exprs;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 8 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        14 ... 31 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        33 ... 39 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        42 ... 47 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 4;
                            continue;
                        }
                        58 ... 132 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        134 ... 159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        161 ... 1631 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        1642 ... 1775 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        1786 ... 1983 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        1994 ... 2405 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2416 ... 2533 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2544 ... 2661 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2672 ... 2789 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2800 ... 2917 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2928 ... 3045 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3056 ... 3173 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3184 ... 3301 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3312 ... 3429 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3440 ... 3557 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3568 ... 3663 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3674 ... 3791 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3802 ... 3871 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3882 ... 4159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        4170 ... 4239 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        4250 ... 5759 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        5761 ... 6111 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6122 ... 6159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6170 ... 6469 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6480 ... 6607 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6618 ... 6783 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6794 ... 6799 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6810 ... 6991 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7002 ... 7087 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7098 ... 7231 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7242 ... 7247 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7258 ... 8191 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        8203 ... 8231 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        8234 ... 8238 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        8240 ... 8286 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        8288 ... 12287 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        12289 ... 42527 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        42538 ... 43215 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43226 ... 43263 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43274 ... 43471 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43482 ... 43503 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43514 ... 43599 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43610 ... 44015 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        44026 ... 65295 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65306 ... 66719 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        66730 ... 69733 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        69744 ... 69871 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        69882 ... 69941 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        69952 ... 70095 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        70106 ... 70383 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        70394 ... 70863 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        70874 ... 71247 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71258 ... 71359 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71370 ... 71471 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71482 ... 71903 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71914 ... 92767 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        92778 ... 93007 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        93018 ... 120781 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        120832 ... 1114111 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 8 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        14 ... 31 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        33 ... 39 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        42 ... 47 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        58 ... 132 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        134 ... 159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        161 ... 1631 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1642 ... 1775 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1786 ... 1983 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1994 ... 2405 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2416 ... 2533 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2544 ... 2661 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2672 ... 2789 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2800 ... 2917 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2928 ... 3045 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3056 ... 3173 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3184 ... 3301 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3312 ... 3429 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3440 ... 3557 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3568 ... 3663 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3674 ... 3791 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3802 ... 3871 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3882 ... 4159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4170 ... 4239 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4250 ... 5759 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        5761 ... 6111 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6122 ... 6159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6170 ... 6469 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6480 ... 6607 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6618 ... 6783 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6794 ... 6799 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6810 ... 6991 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7002 ... 7087 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7098 ... 7231 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7242 ... 7247 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7258 ... 8191 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8203 ... 8231 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8234 ... 8238 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8240 ... 8286 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8288 ... 12287 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        12289 ... 42527 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        42538 ... 43215 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43226 ... 43263 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43274 ... 43471 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43482 ... 43503 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43514 ... 43599 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43610 ... 44015 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        44026 ... 65295 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        65306 ... 66719 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        66730 ... 69733 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69744 ... 69871 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69882 ... 69941 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69952 ... 70095 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70106 ... 70383 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70394 ... 70863 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70874 ... 71247 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71258 ... 71359 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71370 ... 71471 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71482 ... 71903 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71914 ... 92767 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        92778 ... 93007 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        93018 ... 120781 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        120832 ... 1114111 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 8 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        14 ... 31 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        33 ... 39 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        42 ... 47 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        58 ... 132 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        134 ... 159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        161 ... 1631 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1642 ... 1775 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1786 ... 1983 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1994 ... 2405 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2416 ... 2533 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2544 ... 2661 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2672 ... 2789 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2800 ... 2917 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2928 ... 3045 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3056 ... 3173 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3184 ... 3301 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3312 ... 3429 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3440 ... 3557 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3568 ... 3663 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3674 ... 3791 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3802 ... 3871 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3882 ... 4159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4170 ... 4239 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4250 ... 5759 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        5761 ... 6111 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6122 ... 6159 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6170 ... 6469 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6480 ... 6607 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6618 ... 6783 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6794 ... 6799 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6810 ... 6991 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7002 ... 7087 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7098 ... 7231 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7242 ... 7247 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7258 ... 8191 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8203 ... 8231 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8234 ... 8238 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8240 ... 8286 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        8288 ... 12287 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        12289 ... 42527 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        42538 ... 43215 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43226 ... 43263 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43274 ... 43471 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43482 ... 43503 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43514 ... 43599 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43610 ... 44015 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        44026 ... 65295 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        65306 ... 66719 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        66730 ... 69733 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69744 ... 69871 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69882 ... 69941 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69952 ... 70095 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70106 ... 70383 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70394 ... 70863 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70874 ... 71247 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71258 ... 71359 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71370 ... 71471 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71482 ... 71903 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71914 ... 92767 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        92778 ... 93007 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        93018 ... 120781 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        120832 ... 1114111 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    v
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::SExpr(__0))
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Int(__0))
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Box<Expr>
{
    Box::new(Expr::Atom(__0))
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    (__0).to_owned()
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![]
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    v
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![__0]
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action9(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
    )
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action9(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __0,
        __temp0,
    )
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expr>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action7(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action8(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
