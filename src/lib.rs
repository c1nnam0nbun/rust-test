use std::collections::HashSet;

#[ derive( PartialEq, Clone ) ]
pub enum Entry
{
    Value( i32 ),
    Any
}

///Selects among `available` values variants that comply to requirements
/// set by `allowed` and `preferred` slices. `allowed` and `preferred` are slices of type
/// [ Entry ], which can be either [ Entry::Value( i32 ) ] to denote specific value
/// or [ Entry::Any ] to allow all possible values.
///
/// Can return only those values, which are in both `available` and `allowed` slices.
/// If it is impossible to find value, requested in `preferred`, algorithm returns either
/// nearest value, higher than requested, or, if it isn't possible as well, then nearest lower value.
///
/// Returns empty vector if none of the values are available.
///
/// # Examples
/// ```
///
/// use rust_test::{ attempt, Entry };
///
/// let available = [ 240, 720 ];
/// let allowed = [ Entry::Value( 360 ), Entry::Value( 720 ) ];
/// let preferred = [ Entry::Value( 1080 ) ];
///
/// assert_eq!( [ 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
/// ```
///
/// ## With [ Entry::Any ]:
/// ```
///
/// use rust_test::{ attempt, Entry };
///
/// let available = [ 240, 360, 720 ];
/// let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ), Entry::Value( 720 ) ];
/// let preferred = [ Entry::Any, Entry::Value( 720 ) ];
///
/// assert_eq!( [ 240, 360, 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
/// ```
pub fn attempt( available : &[ i32 ], allowed : &[ Entry ], preferred : &[ Entry ]) -> Vec< i32 >
{
    let allowed_any = allowed.contains( &Entry::Any );

    let allowed = if allowed_any
    {
        available.to_vec()
    }
    else
    {
        allowed.iter().map( |e| match e
        {
            Entry::Value( num ) => *num,
            Entry::Any => unreachable!()
        } ).collect()
    };

    let present : Vec< _ > = if allowed_any
    {
        allowed
    }
    else
    {
        available
            .iter()
            .cloned()
            .filter( |num| allowed.contains( num ) )
            .collect()
    };
    
    if present.is_empty() || preferred.contains( &Entry::Any )
    {
        return present
    }

    let mut result = HashSet::< i32 >::new();
    for i in 0 .. preferred.len()
    {
        match &preferred[ i ] {
            Entry::Value( num ) => if present.contains( num )
            {
                result.insert( *num );
            }
            else
            {
                let mut j = 0usize;
                let index = loop
                {
                    if j >= present.len()
                    {
                        break present.len();
                    }
                    if &present[ j ] > num
                    {
                        result.insert( present[ j ] );
                        break j;
                    }
                    j += 1;
                };
                if index == present.len()
                {
                    result.insert( present[ index - 1 ] );
                }
            }
            Entry::Any => unreachable!()
        }

    }
    let mut result : Vec< _ > = result.into_iter().collect();
    result.sort();
    result
}

#[ cfg( test ) ]
mod test
{
    use crate::{ attempt, Entry };

    #[ test ]
    fn example_1()
    {
        let available = [ 240, 360, 720 ];
        let allowed = [ Entry::Value( 360 ), Entry::Value( 720 ) ];
        let preferred = [ Entry::Value( 1080 ) ];

        //NOTE: Example says that expected return value is [ 360 ]
        //However, task states that if algorithm is not able to return preferred value,
        //it should return closest value bigger than preferred (1080 is higher than any of available,
        //so not applicable). If not possible, it should return closest value lower than preferred.
        //And 720 is closer to 1080 than 360, so it should be expected as an output
        assert_eq!( [ 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_2()
    {
        let available = [ 240, 720 ];
        let allowed = [ Entry::Value( 360 ), Entry::Value( 720 ) ];
        let preferred = [ Entry::Value( 1080 ) ];

        assert_eq!( [ 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_3()
    {
        let available = [ 240 ];
        let allowed = [ Entry::Value( 360 ), Entry::Value( 720 ) ];
        let preferred = [ Entry::Value( 1080 ) ];

        assert_eq!( [ 0; 0 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_4()
    {
        let available = [ 240, 360, 720 ];
        let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ), Entry::Value( 720 ), Entry::Value( 1080 ) ];
        let preferred = [ Entry::Value( 240 ), Entry::Value( 360 ) ];

        assert_eq!( [ 240, 360 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_5()
    {
        let available = [ 240, 720 ];
        let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ), Entry::Value( 720 ), Entry::Value( 1080 ) ];
        let preferred = [ Entry::Value( 240 ), Entry::Value( 360 ) ];

        assert_eq!( [ 240, 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_6()
    {
        let available = [ 240, 720 ];
        let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ), Entry::Value( 1080 ) ];
        let preferred = [ Entry::Value( 240 ), Entry::Value( 360 ) ];

        assert_eq!( [ 240 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_7()
    {
        let available = [ 720 ];
        let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ), Entry::Value( 1080 ) ];
        let preferred = [ Entry::Value( 240 ), Entry::Value( 360 ) ];

        assert_eq!( [ 0; 0 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_8()
    {
        let available = [ 240, 360 ];
        let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ) ];
        let preferred = [ Entry::Value( 720 ), Entry::Value( 1080 ) ];

        assert_eq!( [ 360 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_9()
    {
        let available = [ 240, 360, 720 ];
        let allowed = [ Entry::Value( 360 ), Entry::Any ];
        let preferred = [ Entry::Value( 360 ), Entry::Value( 720 ) ];

        assert_eq!( [ 360, 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_10()
    {
        let available = [ 240, 360, 720 ];
        let allowed = [ Entry::Value( 240 ), Entry::Value( 360 ), Entry::Value( 720 ) ];
        let preferred = [ Entry::Any, Entry::Value( 720 ) ];

        assert_eq!( [ 240, 360, 720 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_11()
    {
        let available = [ 240, 360, 720 ];
        let allowed = [ Entry::Value( 360 ), Entry::Value( 1080 ) ];
        let preferred = [ Entry::Any, Entry::Value( 720 ) ];

        assert_eq!( [ 360 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

    #[ test ]
    fn example_12()
    {
        let available = [ 240, 360, 720 ];
        let allowed = [ Entry::Value( 1080 ) ];
        let preferred = [ Entry::Any, Entry::Value( 720 ) ];

        assert_eq!( [ 0; 0 ], attempt( &available, &allowed, &preferred ).as_slice() );
    }

}