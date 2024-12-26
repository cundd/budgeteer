mod amount;
mod currency;
mod date;
mod invoice_type;
mod note;
use std::path::Path;

use dialoguer::console::Style;
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Confirm;

use crate::currency::Currency;
use crate::error::Res;
use crate::file::FileWriter;
use crate::invoice::amount::Amount;
use crate::invoice::Invoice;
use crate::printer::{Printer, PrinterTrait};

use self::amount::read_amount;
use self::currency::read_currency;
use self::date::read_date;
use self::invoice_type::read_invoice_type;
use self::note::NoteWizard;

// Trait for "sub"-wizards
trait WizardTrait<T> {
    fn read(&self, theme: &dyn Theme, invoices: &[Invoice]) -> Res<T>;
}

pub struct Wizard {
    theme: Box<dyn Theme>,
    note_wizard: NoteWizard,
}

impl Wizard {
    pub fn new() -> Wizard {
        let mut theme = ColorfulTheme::default();
        theme.defaults_style = Style::new().blink();

        Wizard {
            theme: Box::new(theme),
            note_wizard: NoteWizard::default(),
        }
    }

    pub fn run<P: AsRef<Path>>(
        &self,
        printer: &Printer,
        base_currency: &Currency,
        output_file: P,
        invoices: &[Invoice],
    ) -> Res<()> {
        println!("Welcome to the invoice wizard");

        println!("Answer the following questions to insert a new invoice");
        println!("(Press ctrl+c to exit)");

        self.run_inner(printer, base_currency, output_file, invoices)
    }

    fn run_inner<P: AsRef<Path>>(
        &self,
        printer: &Printer,
        base_currency: &Currency,
        output_file: P,
        invoices: &[Invoice],
    ) -> Res<()> {
        let invoice = self.create_invoice(invoices)?;

        println!();
        println!("Read the following invoice:");
        printer.print_invoice(base_currency, &invoice);

        let confirm = Confirm::with_theme(self.theme.as_ref());
        if confirm
            .clone()
            .with_prompt("Save this invoice?")
            .default(true)
            .interact()?
        {
            FileWriter::write_invoice(&output_file, &invoice)?;
            println!("Saved the new invoice");

            if confirm
                .with_prompt("Do you want to insert another invoice?")
                .default(true)
                .interact()?
            {
                self.run_inner(printer, base_currency, output_file, invoices)
            } else {
                Ok(())
            }
        } else {
            println!();
            println!("Build another invoice instead");

            self.run_inner(printer, base_currency, output_file, invoices)
        }
    }

    fn create_invoice(&self, invoices: &[Invoice]) -> Res<Invoice> {
        let theme = self.theme.as_ref();
        let date = read_date(theme)?;
        let currency = read_currency(theme)?;
        let amount = read_amount(theme)?;
        let invoice_type = read_invoice_type(theme)?;
        let note = self.note_wizard.read(theme, invoices)?;

        Ok(Invoice::new(
            date,
            Amount::new(amount, &currency),
            None,
            invoice_type,
            Some(note),
        ))
    }
}
