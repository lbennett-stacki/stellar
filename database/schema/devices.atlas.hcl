table "devices" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "user_id" {
    type = uuid
  }
  foreign_key "user_fk" {
    columns     = [column.user_id]
    ref_columns = [table.users.column.id]
  }

  column "password" {
    type = varchar(255)
  }

  column "created_at" {
    type = date
  }

  column "updated_at" {
    type = date
    null = true
  }
}
