pub mod plot
{
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    pub enum PlotType
    {
      IncomeVsExpenses,
      IncomePerCategory,
      ExpensesPerCategory,
      WealthGrowth,
    }

    /*impl PlotType
    {
        fn as_str(&self) -> &static str
        {
            match *self
            {
                PlotType::IncomeVsExpenses => "IncomeVsExpenses",
                PlotType::IncomePerCategory => "IncomePerCategory",
                PlotType::ExpensesPerCategory => "ExpensesPerCategory",
                PlotType::WealthGrowth => "WealthGrowth",
            }
        }
    }*/

    impl FromStr for PlotType
    {
        type Err = ();

        fn from_str(a_str: &str) -> Result<Self, Self::Err>
        {
            match a_str
            {
                "IncomeVsExpenses" => Ok(PlotType::IncomeVsExpenses),
                "IncomePerCategory" => Ok(PlotType::IncomePerCategory),
                "ExpensesPerCategory" => Ok(PlotType::ExpensesPerCategory),
                "WealthGrowth" => Ok(PlotType::WealthGrowth),
                _ => Err(()),
            }
        }
    }
}
