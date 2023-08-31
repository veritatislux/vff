fn main()
{
    if let Err(error) = vff::read_cmdline_args()
    {
        println!("{}", error);
    }
}
